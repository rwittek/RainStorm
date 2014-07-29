use {Cheat, GamePointers};
use sdk;
use core;
use core::prelude::*;
use sdk::Animating;
use sdk::Entity;
use sdk::TFPlayer;
use sdk::BaseObject;
use sdk::OnTeam;
use sdk::{Scout, Soldier, Pyro, Demoman, Heavy, Engineer, Medic, Sniper, Spy, TFClass};

#[deriving(Show)]
pub enum AimbotTargetType {
	Player,
	Sentry,
	Teleporter,
	/* poot */ Dispenser, /* here */
	MVMTank,
}
fn get_target_type<EntType: Entity>(ptrs: &GamePointers, ent: EntType) -> Option<AimbotTargetType> {
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

pub struct Aimbot {
	enabled: bool,
	hitbox: Option<i32>,
	stop_firing: u8,
	
	fovweight: f32,
	distweight: f32,
	
	lastaim: Option<(sdk::Vector, Option<sdk::Vector>)>,
	predict: bool
}

impl Aimbot {
	fn find_target_spot(&mut self, ptrs: &GamePointers, viewangles: &sdk::QAngle) -> Option<sdk::Vector> {
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		let me: TFPlayer = unsafe { Entity::from_ptr( ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap())};

		let mut direction = sdk::Vector::new();

		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			eyes.x += (eye_offsets)[0];
			eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		let mut max_priority = core::f32::MIN_VALUE; // this is signed
		let mut best_targ: Option<(sdk::raw::C_BaseEntityPtr, sdk::Vector)> = None;
		let mut ivengineclient = ptrs.ivengineclient;
		let mut icliententitylist = ptrs.icliententitylist;
		let mut ienginetrace = ptrs.ienginetrace;
		
		{
			let current_aim_norm = viewangles.to_vector().norm();
			let prioritize = |pos: sdk::Vector, ent: &sdk::Entity, hitbox: Option<i32>, targtype: AimbotTargetType| {
				
				let aimvec = pos - eyes;
				let mut tempangles = aimvec.to_angle();
				
				// can we actually see this?
				match sdk::utils::trace_to_entity(ivengineclient, icliententitylist, ienginetrace, &tempangles) {
					Some((trace_ent, hit_hitbox)) if trace_ent == ent.get_ptr() => {
						match hitbox {
							Some(hb) => {
								if hb != hit_hitbox {
									return; // we see it, but it's the wrong hitbox!
								}
							},
							None => ()
						}
					},
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
						match player.get_class() {
							Scout => if dist < 1000.0 { 100.0 } else { -1000.0 }, // scouts are only really dangerous up close
							Soldier => 0.0,
							Pyro => if dist < 500.0 { 1000.0 } else { -1000.0 },
							Demoman => 0.0,
							Heavy => if dist < 2000.0 { 10000.0 } else { -1000.0 },
							Engineer => -200.0, // engineers aren't really a threat
							Medic => 400.0, // kill meds first
							Sniper => 3000.0 + dist, // snipers are scary even from far away; ignore distance
							Spy => if dist < 300.0 { 10000.0 } else { -1000.0 }, // backstab THIS
						}
					},
					Dispenser | Teleporter => -5000.0,
					Sentry => if dist <= 1500.0 { 10000.0 } else { -5000.0 }, // sentries have 1024HU range
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
						let player: sdk::TFPlayer = unsafe {
							Entity::from_ptr(ent.get_ptr())
						};
							
						match self.hitbox {
							Some(hitbox) => {
								
								let hitbox_pos = player.get_hitbox_position(ptrs.ivmodelinfo, hitbox);
			
								prioritize(hitbox_pos, &ent as &Entity, Some(hitbox), targtype)
							},
							None => {
								for hitbox_pos in sdk::utils::HitboxPositionIterator::new(player, ptrs.ivmodelinfo) {
									prioritize(hitbox_pos, &ent as &Entity, None, targtype)
								}
							}
						}
					},
					Sentry | Teleporter | Dispenser | MVMTank => {
						unsafe {
							prioritize(ent.worldspacecenter(), &ent as &Entity, None, targtype)
						}
					},
					
				}
			}
		}
		
		best_targ.map(|(ent, pos)| {
			pos
		})
		
		/*+ {
				(ent.get_velocity() - me.get_velocity()).scale(unsafe { sdk::raw::get_current_latency(ptrs.ivengineclient.get_ptr()) })
			}
		})*/
	}
			
	fn aim_at_target(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd, target: sdk::Vector) {
		let localplayer_entidx = unsafe {ptrs.ivengineclient.get_local_player()};
		let me = unsafe {ptrs.icliententitylist.get_client_entity(localplayer_entidx)}.unwrap();

		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			eyes.x += (eye_offsets)[0];
			eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		let aimvec = target - eyes;
		
		//let predicted = aimvec + delta_p.scale(unsafe { 
		//
		//} * 1000.0 / 66.0);
	
		cmd.viewangles = aimvec.to_angle();
		
	}
}

impl Cheat for Aimbot {
	fn new() -> Aimbot {
		Aimbot {
			enabled: false,
			hitbox: None,
			stop_firing: 1,
			
			// these values are not remotely on the same scale
			fovweight: 500.0,
			distweight: 1.0,
			
			predict: true,
			lastaim: None
		}
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Aimbot"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		if cmd.buttons & sdk::IN_ATTACK == 0 {
			return; // not attacking, who cares
		}
		
		let maybe_targspot = self.find_target_spot(ptrs, &cmd.viewangles);
		match maybe_targspot {
			Some(targspot) => { self.aim_at_target(ptrs, cmd, targspot) },
			None => {
				if self.stop_firing != 0 {
					cmd.buttons = cmd.buttons & (!sdk::IN_ATTACK)
				};
				return
			} // nothing to aim at
		}
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
			"predict" => {
				self.predict = ::utils::str_to_integral::<u32>(val[0]) != 0;
				log!("Prediction: {}\n", self.predict);
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