use {Cheat, GamePointers};
use sdk;
use core;
use core::prelude::*;

pub struct Aimbot {
	enabled: bool,
	hitbox: Option<i32>
}

impl Aimbot {
	fn find_target_spot(&mut self, ptrs: &GamePointers, viewangles: &sdk::QAngle) -> Option<sdk::Vector> {
		let localplayer_entidx = unsafe {ptrs.ivengineclient.to_option().unwrap().get_local_player()};
		let local_baseentity = unsafe {ptrs.icliententitylist.to_option().unwrap().get_client_entity(localplayer_entidx)};
		
		let me: &mut sdk::C_BaseEntity = if local_baseentity.is_not_null() {
			unsafe { core::mem::transmute(local_baseentity) }
		} else {
			quit!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); 
		};

		let mut direction = sdk::Vector::new();

		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			eyes.x += (eye_offsets)[0];
			eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		let mut maxdist = 100000.0;
		let mut best_targ: Option<sdk::Vector> = None;
		
		let mut ivengineclient = unsafe { ptrs.ivengineclient.to_option().unwrap() };
		let mut icliententitylist = unsafe { ptrs.icliententitylist.to_option().unwrap() };
		let mut ienginetrace = unsafe { ptrs.ienginetrace.to_option().unwrap() };
		
		let mut tempangles = sdk::QAngle { pitch: 0.0, yaw: 0.0, roll: 0.0 };
		
		::utils::map_all_players(ptrs.icliententitylist, |ptr| {
			if unsafe { (*ptr).get_team() == me.get_team() } {
				// teammates
				return;
			}
			if unsafe { (*ptr).get_life_state() != 0} {
				//log!("Entity is dead: {}\n", unsafe { *((*ptr).ptr_offset::<i8>(0x00A1)) });
				return;
			}
			
			let pos: sdk::Vector = match self.hitbox {
				Some(hitbox) => {
					let mut pos = sdk::Vector { x: 0.0, y: 0.0, z: 0.0 };
					unsafe {
						sdk::c_baseanimating_gethitboxposition(&*(ptr as *const sdk::C_BaseAnimating), ptrs.ivmodelinfo, hitbox, &mut pos, viewangles)
					};
					pos
				},
				None => { unsafe {
						(*ptr).worldspacecenter()
				} }
			};
			let aimvec = pos - eyes;
			unsafe { sdk::vector_angles(&aimvec, &mut tempangles) };
			// can we actually see this?
			if !::utils::should_shoot(ivengineclient, icliententitylist, ienginetrace, &tempangles, None) {
				// can't see it
				return;
			}
			// TODO: priority
			let dist = (aimvec).length();
			if dist < maxdist {
				//log!("target: {}, {}, {}", unsafe {(*ptr).get_index()}, pos, dist);
				maxdist = dist;
				best_targ = Some(pos);
			}
		});
		//log!("best target: {}\n", best_targ);
		best_targ
	}		
			
			
	fn aim_at_target(&self, ptrs: &GamePointers, viewangles: &mut sdk::QAngle, target: sdk::Vector) {
		let localplayer_entidx = unsafe {ptrs.ivengineclient.to_option().unwrap().get_local_player()};
		let local_baseentity = unsafe {ptrs.icliententitylist.to_option().unwrap().get_client_entity(localplayer_entidx)};
		
		let me: &mut sdk::C_BaseEntity = if local_baseentity.is_not_null() {
			unsafe { core::mem::transmute(local_baseentity) }
		} else {
			quit!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); 
		};

		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			eyes.x += (eye_offsets)[0];
			eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		
		let aimvec = target - eyes;
		unsafe {
			sdk::vector_angles(&aimvec, viewangles);
		}
	}
}

impl Cheat for Aimbot {
	fn new() -> Aimbot {
		Aimbot { enabled: false, hitbox: None }
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
			Some(targspot) => { self.aim_at_target(ptrs, &mut cmd.viewangles, targspot) },
			None => {cmd.buttons = cmd.buttons & (!sdk::IN_ATTACK); return} // nothing to aim at
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
			_ => {}
		}
	}
}