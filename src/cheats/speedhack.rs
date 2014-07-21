use {Cheat, GamePointers};
use sdk;

pub struct Speedhack {
	enabled: bool
}

impl Cheat for Speedhack {
	fn new() -> Speedhack {
		Speedhack { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Speedhack"
	}
	fn process_usercmd(&mut self, _ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		use sdk::{IN_ATTACK, IN_FORWARD, IN_BACK, IN_MOVELEFT, IN_MOVERIGHT};
		
		if !self.enabled {
			return;
		}
		
		if (cmd.buttons & IN_ATTACK) == 0 {
			use core::f32::consts::PI;
			use core::intrinsics::{fabsf32};
			// maybe we can go fast! yay!
			
			let (mut forward, mut side) = (0f32, 0f32);
			let mut can_move = false; // true if at least one movement key held down
			
			
			if (cmd.buttons & IN_FORWARD != 0 ) { forward = 1.0; can_move = true; }
			if (cmd.buttons & IN_BACK != 0 ) { if forward != 0.0 { can_move = false } else { forward = -1.0; can_move = true; }}
			if (cmd.buttons & IN_MOVELEFT != 0) { side = 1.0; can_move = true; }
			if (cmd.buttons & IN_MOVERIGHT != 0 ) { if side != 0.0 { can_move = false } else {side = -1.0; can_move = true; }}
			
			// remember high-school trig?
			let rotang = unsafe { ::cmath::atan2f(side, forward) };

			if !can_move {
				return;
			}
			

			cmd.forwardmove = -999f32;
			
			cmd.viewangles.pitch = 89f32;
			cmd.viewangles.yaw = (cmd.viewangles.yaw + 180f32 + (180f32 * rotang / PI)) % 360f32; // flip us around
			cmd.viewangles.roll = 90f32; // apparently not capped, ggnore
		}
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}