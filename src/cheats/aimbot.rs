use {Cheat, GamePointers};
use sdk;
use core;
use core::prelude::*;

#[deriving(Show)]
pub enum AimbotTargetType {
	Player,
	Sentry,
	Teleporter,
	/* poot */ Dispenser, /* here */
	MVMTank
}
fn get_target_type(ptrs: &GamePointers, ent: sdk::C_BaseEntity) -> Option<AimbotTargetType> {
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
	distweight: f32
}

impl Aimbot {
	fn find_target_spot(&mut self, ptrs: &GamePointers, viewangles: &sdk::QAngle) -> Option<sdk::Vector> {
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		let me = ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap();

		let mut direction = sdk::Vector::new();

		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			eyes.x += (eye_offsets)[0];
			eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		let mut max_priority = core::f32::MIN_VALUE; // this is signed
		let mut best_targ: Option<(i32, sdk::Vector)> = None;
		let mut ivengineclient = ptrs.ivengineclient;
		let mut icliententitylist = ptrs.icliententitylist;
		let mut ienginetrace = ptrs.ienginetrace;
		
		{
			let current_aim_norm = viewangles.to_vector().norm();
			let prioritize = |pos: sdk::Vector, ent: sdk::C_BaseEntity, hitbox: Option<i32>, targtype: AimbotTargetType| {
				
				let aimvec = pos - eyes;
				let mut tempangles = aimvec.to_angle();
				
				// can we actually see this?
				match sdk::utils::trace_to_entity(ivengineclient, icliententitylist, ienginetrace, &tempangles) {
					Some((trace_ent, hit_hitbox)) if trace_ent == ent => {
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
				
				let distpriority = -1.0 * self.distweight * (aimvec).length(); // farther away = worse target
				let fovpriority = self.fovweight * aimvec.norm().dotproduct(&current_aim_norm); // closer to current crosshair position = better target
				let priority = distpriority + fovpriority;
				//log!("priority: {} from {}, {}\n", priority, distpriority, fovpriority);
				if priority > max_priority {
					//log!("target: {}, {}, {}", unsafe {(*ptr).get_index()}, pos, dist);
					max_priority = priority;
					best_targ = Some((ent.get_index(), pos));
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
					.filter(|&(ptr, targtype)|  ptr.get_team() != me.get_team() ) // only enemies
					.filter(|&(ptr, targtype)|  ptr.get_life_state() == 0 ) { // only alive entities
					
				match targtype {
					Player => { 	
						// FIXME: Ew.
						let baseanimating = unsafe {
							sdk::C_BaseAnimating::from_ptr(sdk::raw::C_BaseAnimatingPtr::from_uint(ent.get_ptr().to_uint()))
						};
							
						match self.hitbox {
							Some(hitbox) => {
								
								let hitbox_pos = baseanimating.get_hitbox_position(ptrs.ivmodelinfo, hitbox);
			
								prioritize(hitbox_pos, ent, Some(hitbox), targtype)
							},
							None => {
								for hitbox_pos in sdk::utils::HitboxPositionIterator::new(baseanimating, ptrs.ivmodelinfo) {
									prioritize(hitbox_pos, ent, None, targtype)
								}
							}
						}
					},
					Sentry | Teleporter | Dispenser | MVMTank => {
						unsafe {
							prioritize(ent.worldspacecenter(), ent, None, targtype)
						}
					},
					
				}
			}
		}
		
		best_targ.map(|(entidx, pos)| pos)
	}
			
	fn aim_at_target(&self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd, target: sdk::Vector) {
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
			distweight: 1.0
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