use {Cheat, GamePointers};
use core;
use core::prelude::*;
use sdk;
use sdk::Animating;
use sdk::Entity;
use sdk::TFPlayer;
use sdk::BaseObject;
use sdk::OnTeam;
use sdk::{Scout, Soldier, Pyro, Demoman, Heavy, Engineer, Medic, Sniper, Spy};
use sdk::utils;
use sdk::Float;
use libc;


#[deriving(Show)]
pub enum AimbotTargetType {
	Player,
	Sentry,
	Teleporter,
	/* poot */ Dispenser, /* here */
	MVMTank,
}


/// Given an entity, determine what type of aimbot target it is.
fn get_target_type<EntType: Entity>(_ptrs: &GamePointers, ent: EntType) -> Option<AimbotTargetType> {
	let classname = ent.get_classname();
	
	match classname {
		"CTFPlayer" => Some(Player),
		"CObjectSentrygun" => Some(Sentry),
		"CObjectTeleporter" => Some(Teleporter),
		"CObjectDispenser" => Some(Dispenser),
		"CTFTankBoss" => Some(MVMTank),
		_ => None
	}
}

pub enum AimbotMode {
	SemiSilent,
	Sticky(f32)
}

pub struct Aimbot {
	enabled: bool,
	hitbox: Option<i32>,
	stop_firing: u8,
	
	// Ugly scaling factors that should be improved
	fovweight: f32,
	distweight: f32,
	
	interpdata: Option<(i32, i32, sdk::Vector, sdk::Vector)>,
	
	mode: AimbotMode,
	
	lasttick: i32
}

impl Aimbot {

	/// Find the best thing to shoot at... if there is one.
	/// Otherwise, returns None.
	fn find_target(&mut self, ptrs: &GamePointers, viewangles: &sdk::QAngle, time: f32) -> Option<(i32, sdk::Vector)> {
		let me: TFPlayer = unsafe { Entity::from_ptr( utils::get_local_player_entity(ptrs)) };
		
		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			//eyes.x += (eye_offsets)[0];
			//eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		let mut max_priority = core::f32::MIN_VALUE; // this is signed
		let mut best_targ: Option<(sdk::raw::C_BaseEntityPtr, sdk::Vector)> = None;
		
		{
			let current_aim_norm = viewangles.to_vector().norm();
			let prioritize = |pos: sdk::Vector, ent: &sdk::Entity, hitbox: Option<i32>, targtype: AimbotTargetType| {
				
				let aimvec = pos - eyes;
				let tempangles = aimvec.to_angle();
				
				match sdk::utils::trace_to_entity(ptrs, &tempangles, 0x200400B) {
					Some((trace_ent, hit_hitbox)) if trace_ent.get_index() == ent.get_index() => {
						match hitbox {
							Some(hb) => {
								if hb != hit_hitbox {
									//log!("HIT WRONG HITBOX!\n");
									return; // we see it, but it's the wrong hitbox!
								}
							},
							None => ()
						}
					},
					Some((trace_ent, hit_hitbox)) => { // Wrong entity!
						//log!("Wrong entity (hit {} at {})!\n", trace_ent.get_classname(), hit_hitbox);
						return;
					}
					_ => {
						return
					}
				}

				let dist = aimvec.length(); // farther away = worse target
				//log!("Aimvec distance: {}\n", dist);
				let fovpriority = self.fovweight * (1.0 + aimvec.norm().dotproduct(&current_aim_norm)); // closer to current crosshair position = better target
				let distpriority = -1.0 * self.distweight * dist;
				let targtypepriority = match targtype {
					Player => {
						let player: TFPlayer = unsafe { Entity::from_ptr(ent.get_ptr()) };
						let classpriority = match player.get_class() {
							Scout => 0.0,
							Soldier => 0.0,
							Pyro => 0.0,
							Demoman => 0.0,
							Heavy => 0.0,
							Engineer => 0.0,
							Medic => 300.0, // kill meds first
							Sniper => dist, // snipers are scary even from far away; ignore distance
							Spy => if dist < 500.0 { 1000.0 } else { -1000.0 }, // backstab THIS
						};
						let healthpriority = 0.0; // 10.0 * (150 - player.get_health()) as f32;
						
						classpriority + healthpriority
					},
					Dispenser | Teleporter => -5000.0,
					Sentry => if dist <= 1500.0 { 10000.0 } else { -100.0 }, // sentries have 1024HU range
					MVMTank => 0.0, // I don't play enough MVM to know what to put here
				};

				let priority = targtypepriority + distpriority + fovpriority;
				//log!("priority: {} from {}, {}, {}\n", priority, distpriority, fovpriority, targtypepriority);
				if priority > max_priority {
					//log!("target: {}\n", unsafe {ent.get_index()});
					max_priority = priority;
					best_targ = Some((ent.get_ptr(), pos));
				}
			};
			
			for (ent, targtype) in sdk::utils::EntityIterator::new(ptrs.icliententitylist)
					.map(|ent| (ent, get_target_type(ptrs, ent)))
					.filter_map(|(ent, maybe_targtype)| {
						match maybe_targtype {
							Some(targtype) => Some((ent, targtype)),
							None => None
						}
					})
					.filter(|&(ptr, targtype)| match targtype {
						Player => {
							let player: TFPlayer = unsafe { Entity::from_ptr(ptr) };
							
							player.get_team() != me.get_team() && player.get_life_state() == 0
						},
						Sentry | Dispenser | Teleporter => {
							let object: BaseObject = unsafe { Entity::from_ptr(ptr) };
							object.get_team() != me.get_team()
						},
						_ => true
					}) {

				match targtype {
					Player => { 	
						let mut player: sdk::TFPlayer = unsafe {
							Entity::from_ptr(ent.get_ptr())
						};
						log!("Interpolating... ");
						player.interpolate(0.0);
						log!("OK.\n");
						match self.hitbox {
							Some(hitbox) => {
								
								let hitbox_pos = player.get_hitbox_position(ptrs.ivmodelinfo, hitbox);
								prioritize(hitbox_pos, &ent as &Entity, None, targtype)
							},
							None => {
								for hitbox_pos in sdk::utils::HitboxPositionIterator::new(player, ptrs.ivmodelinfo) {
									prioritize(hitbox_pos, &ent as &Entity, None, targtype)
								}
							}
						}
					},
					Sentry | Teleporter | Dispenser | MVMTank => {
						prioritize(ent.worldspacecenter(), &ent as &Entity, None, targtype)
					},
					
				}
			}
		}
		
		best_targ.map(|(ent, pos)| (ent.get_index(), pos))
		
		/*+ {
				(ent.get_velocity() - me.get_velocity()).scale(unsafe { sdk::raw::get_current_latency(ptrs.ivengineclient.get_ptr()) })
			}
		})*/
	}
			
	fn aim_at_target(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd, target: Option<sdk::Vector>) {	
		use cmath;
		
		let me = utils::get_local_player_entity(ptrs);
		
		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			//eyes.x += (eye_offsets)[0];
			//eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		
		match self.mode {
			SemiSilent => {
				if cmd.buttons & sdk::IN_ATTACK == 0 {
					return; // not attacking, who cares
				}
				match target {
					Some(target) => {
						let aimvec = target - eyes;
						
						let oldviewangles = cmd.viewangles;
						cmd.viewangles = aimvec.to_angle();
						
						let (forwardmove, sidemove, upmove) = sdk::utils::rotate_movement((cmd.forwardmove, cmd.sidemove, cmd.upmove),
								oldviewangles, cmd.viewangles);
						cmd.forwardmove = forwardmove; cmd.sidemove = sidemove; cmd.upmove = upmove;
					},
					None => ()
				}
			},
			
			Sticky(base_sens) => {
				let mut sens_var = ptrs.icvar.expect("bad ICvar\n").find_var("sensitivity").expect("sens not found!");
				
				let sensscale = match target {
					Some(target) => {
						let aimvec = target - eyes;
						
						let currentaim = cmd.viewangles.to_vector();
						let cos_fov = aimvec.norm().dotproduct(&currentaim.norm());
 
						::utils::clamp(unsafe { (25.0 * core::num::abs(cmath::acosf(cos_fov)) - 1.0).exp() },
							0.2,
							1.5
						)
					},
					None => 1.0
				};
				
				sens_var.setvalue(Float(base_sens * sensscale));
			}
		}
	}

	fn modify_cmd(&mut self, ptrs: &GamePointers, mut cmd: sdk::CUserCmd) -> sdk::CUserCmd {
		let maybe_target = self.find_target(ptrs, &cmd.viewangles, cmd.tick_count as f32 / 66.0);
		let predicted_target = maybe_target.map(|(hb, pos)| pos); // self.predict(ptrs, cmd.tick_count, maybe_target);
		self.aim_at_target(ptrs, &mut cmd, predicted_target);
		match predicted_target {
			Some(target) => (),
			None => { // nothing to aim at
				if self.stop_firing != 0 {
					cmd.buttons &= (!sdk::IN_ATTACK)
				}
			} 
		}
		
	
		cmd
	}
}


impl Cheat for Aimbot {
	fn new() -> Aimbot {
		Aimbot {
			enabled: false,
			hitbox: None,
			stop_firing: 1,
			
			// these values are not remotely on the same scale
			fovweight: 0.0,
			distweight: 1.0,
			
			interpdata: None,
			
			mode: SemiSilent,
			
			lasttick: 0,
		}
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Aimbot"
	}

	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let me: TFPlayer = unsafe { Entity::from_ptr( utils::get_local_player_entity(ptrs)) };
		
		utils::predict(ptrs, cmd);
		*cmd = self.modify_cmd(ptrs, *cmd);
		//ptrs.ivengineclient.set_viewangles(&cmd.viewangles);
	}

	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
	fn set_config(&mut self, var: &str, val: &[&str]) {
		match var {
			"hitbox" => {
				self.hitbox = match val[0] {
					"all" => None,
					number => Some(::utils::str_to_integral(number)),
				};
				log!("Hitbox: {}\n", self.hitbox);
			},
			"stop_firing" => {
				self.stop_firing = ::utils::str_to_integral(val[0]);
				log!("Stop firing: {}\n", self.stop_firing);
			}
			"distweight" => {
				self.distweight = ::utils::str_to_integral::<u32>(val[0]) as f32;
				log!("Distance weight: {}\n", self.distweight);
			}
			"fovweight" => {
				self.fovweight = ::utils::str_to_integral::<u32>(val[0]) as f32;
				log!("FOV weight: {}\n", self.fovweight);
			}
			_ => {}
		}
	}
}