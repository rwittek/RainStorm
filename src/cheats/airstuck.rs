use {Cheat, GamePointers};
use sdk;

pub struct Airstuck {
	enabled: bool
}

impl Cheat for Airstuck {
	fn new() -> Airstuck {
		Airstuck { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Airstuck"
	}
	fn process_usercmd(&mut self, _ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		use sdk::IN_ATTACK;
		
		if !self.enabled {
			return;
		}
		
		if (cmd.buttons & IN_ATTACK) == 0 {
			
			cmd.viewangles.pitch = 0.0;
			cmd.viewangles.yaw = 0.0;
			cmd.viewangles.roll = 90.0; // apparently not capped, ggnore
		}
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}