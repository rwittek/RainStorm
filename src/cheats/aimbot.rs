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
	
	last_interpdata: Option<(f32, sdk::Vector)>,
	last_last_interpdata: Option<(f32, sdk::Vector)>
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
		let mut best_targ: Option<sdk::Vector> = None;
		
		let mut ivengineclient = ptrs.ivengineclient;
		let mut icliententitylist = ptrs.icliententitylist;
		let mut ienginetrace = ptrs.ienginetrace;
		
		{
			let prioritize = |pos: sdk::Vector, ent: sdk::C_BaseEntity, targtype: AimbotTargetType| {
				let aimvec = pos - eyes;
				let mut tempangles = aimvec.to_angle();
				
				// can we actually see this?
				match sdk::utils::trace_to_entity(ivengineclient, icliententitylist, ienginetrace, &tempangles) {
					Some(trace_ent) if trace_ent == ent => (), // OK
					Some(trace_ent) => {
						return
					},
					None => {
						return
					}
				}

				let dist = (aimvec).length();
				let priority = -dist;
				if priority > max_priority {
					//log!("target: {}, {}, {}", unsafe {(*ptr).get_index()}, pos, dist);
					max_priority = priority;
					best_targ = Some(pos);
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
			
								prioritize(hitbox_pos, ent, targtype)
							},
							None => {
								for hitbox_pos in sdk::utils::HitboxPositionIterator::new(baseanimating, ptrs.ivmodelinfo) {
									prioritize(hitbox_pos, ent, targtype)
								}
							}
						}
					},
					Sentry | Teleporter | Dispenser | MVMTank => {
						unsafe {
							prioritize(ent.worldspacecenter(), ent, targtype)
						}
					},
					
				}
			}
		}
		
		best_targ
		/*match best_targ {
			Some(best_targ) => {
				//log!("best target: {}\n", best_targ);
				let interped_target = match self.last_interpdata {
					Some((last_time, last_targ)) => { match self.last_last_interpdata {
						Some((last_last_time, last_last_targ)) => {
							let delta_t = last_time - last_last_time;
							let delta_p = last_targ - last_last_targ;
							
							let latency = unsafe { sdk::get_current_latency(ptrs.ivengineclient) };
							log!("latency: {}\n", latency);
							Some(best_targ - (delta_p.scale( latency / delta_t )))
						}, 
						None => {
							log!("meow!\n");
							self.last_last_interpdata = self.last_interpdata;
							self.last_interpdata = Some(( unsafe {
								(*ptrs.ivengineclient).time() }, best_targ));
							
							None
						}
					}},
					None => {
						log!("woof!\n");
						self.last_interpdata = Some(( unsafe {(*ptrs.ivengineclient).time() }, best_targ));
						None
					}
				};
		
				interped_target
			},
			None => None
		}*/
	
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
		Aimbot { enabled: false, hitbox: None, stop_firing: 1, last_interpdata: None, last_last_interpdata: None }
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
				self.hitbox = Some(::utils::str_to_integral(val[0]));
				log!("Hitbox: {}\n", self.hitbox);
			},
			"stop_firing" => {
				self.stop_firing = ::utils::str_to_integral(val[0]);
				log!("Stop firing: {}\n", self.stop_firing);
			}
			_ => {}
		}
	}
}