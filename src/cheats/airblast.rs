use core::prelude::*;
use Cheat;
use GamePointers;

use sdk;
use sdk::{Entity, TFPlayer, OnTeam, utils};

pub struct Airblast {
	enabled: bool,
}

impl Cheat for Airblast {
	fn new() -> Airblast {
		Airblast { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Airblast"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let me: TFPlayer = unsafe { Entity::from_ptr( utils::get_local_player_entity(ptrs)) };
		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
			//eyes.x += (eye_offsets)[0];
			//eyes.y += (eye_offsets)[1];
			eyes.z += (eye_offsets)[2];
		}
		
		let old_buttons = cmd.buttons;
		cmd.buttons = cmd.buttons & !(1<<11);
		utils::predict(ptrs, cmd);
		cmd.buttons = old_buttons;
		
		for ent in sdk::utils::EntityIterator::new(ptrs.icliententitylist) {
			let class = ent.get_classname();
			match class {
				"CTFProjectile_Rocket" | "CTFProjectile_Flare"
				| "CTFProjectile_EnergyBall" | "CTFProjectile_HealingBolt" 
				| "CTFProjectile_Arrow" | "CTFProjectile_SentryRocket" 
				| "CTFProjectile_Throwable" | "CTFThrowable" 
				| "CTFProjectile_Cleaver"  | "CTFProjectile_JarMilk" 
				| "CTFProjectile_Jar" | "CTFStunBall" 
				| "CTFGrenadePipebombProjectile" | "CTFBall_Ornament" => {
					if ent.get_team() != me.get_team() {
						// project
						let latency = unsafe { sdk::raw::get_current_latency(ptrs.ivengineclient.get_ptr(), 3) };
						let mypos = eyes; //+ (me.get_velocity().scale(latency));
						let vel = ent.get_velocity();
						//log!("vel: {} lat: {}\n", vel, latency);
						let corr = vel.scale(latency);
						//log!("corr: {}\n", corr);
						let pos = ent.get_origin() + corr;
						
						let ray = pos - mypos;
						if ray.length() < 200.0 {
							cmd.buttons |= (1<<11);
							let oldviewangles = cmd.viewangles;
							cmd.viewangles = ray.to_angle();
						
							let (forwardmove, sidemove, upmove) = sdk::utils::rotate_movement((cmd.forwardmove, cmd.sidemove, cmd.upmove),
								oldviewangles, cmd.viewangles);
							cmd.forwardmove = forwardmove; cmd.sidemove = sidemove; cmd.upmove = upmove;
						}
					}
				},
				_ => ()
			}
		}		
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}

impl Airblast {
}