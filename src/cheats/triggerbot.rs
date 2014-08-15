use core::prelude::*;
use Cheat;
use GamePointers;

use sdk;
use sdk::{Entity, TFPlayer, OnTeam, utils};

pub struct Triggerbot {
	enabled: bool,
	
	smoothing: u32,
	smoothing_state: u32
}

impl Cheat for Triggerbot {
	fn new() -> Triggerbot {
		Triggerbot { enabled: false, smoothing: 0, smoothing_state: 0 }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Triggerbot"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		
		// button 1 = IN_ATTACK
		if cmd.buttons & 1 == 1 {
			cmd.buttons = !((!cmd.buttons) | 1); // zero the IN_ATTACK bit

			let trace = sdk::utils::trace_to_entity_hitbox(ptrs, &cmd.viewangles);
			match trace {
				Some((ent, hitbox)) if ent.get_classname() == "CTFPlayer" => { // player
					let ent: TFPlayer = unsafe { Entity::from_ptr(ent.get_ptr()) };
					let me: TFPlayer = unsafe { Entity::from_ptr( utils::get_local_player_entity(ptrs)) };
					
					if me.get_team() == ent.get_team() {
						self.smoothing_state += 1;
						if self.smoothing_state > self.smoothing {
							cmd.buttons = cmd.buttons | 1; // set IN_ATTACK
						}
					} else {
						if self.smoothing_state > 0 {
							self.smoothing_state -= 1;
						}
					}
				},
				_ => {
					if self.smoothing_state > 0 {
						self.smoothing_state -= 1;
					}
				}
			}
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
	fn set_config(&mut self, var: &str, val: &[&str]) {
		match var {
			"smoothing" => {
				self.smoothing = ::utils::str_to_integral(val[0]);
				log!("Smoothing: {}\n", self.smoothing);
			},
			_ => {}
		}
	}
}

impl Triggerbot {
}