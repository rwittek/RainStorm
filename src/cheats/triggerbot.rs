use Cheat;
use GamePointers;
use sdk;
use libc;
use core::prelude::*;
use core;

pub struct Triggerbot {
	enabled: bool,
	
	smoothing: u32,
	smoothing_state: u32
}

extern "C" fn should_hit_entity(ent: *const sdk::IHandleEntity, contentsmask: i32) -> bool {
	false
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
		
		let mut ivengineclient = unsafe { ptrs.ivengineclient.to_option().unwrap() };
		let mut icliententitylist = unsafe { ptrs.icliententitylist.to_option().unwrap() };
		let mut ienginetrace = unsafe { ptrs.ienginetrace.to_option().unwrap() };
		// button 1 = IN_ATTACK
		if cmd.buttons & 1 == 1 {
			cmd.buttons = !((!cmd.buttons) | 1); // zero the IN_ATTACK bit
			
			// FIXME
			if false { //::utils::should_shoot(ivengineclient, icliententitylist, ienginetrace, &cmd.viewangles, None) {
				self.smoothing_state = self.smoothing_state + 1;
				if self.smoothing_state > self.smoothing {
					cmd.buttons = cmd.buttons | 1; // set IN_ATTACK
				}
			} else {
				if self.smoothing_state > 0 {
					self.smoothing_state = self.smoothing_state - 1;
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