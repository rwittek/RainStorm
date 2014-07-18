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
		if !self.enabled {
			return;
		}
		
		// FIXME: hook input, see which way we want to go
		// atm, we can't move backwards which is annoying,
		// and moving sideways gets regular speed
		if (cmd.buttons & (1 << 0)) == 0 && (cmd.forwardmove > 0.1f32 || cmd.forwardmove < -0.001f32) {
			// speedhack time
			cmd.forwardmove = -999f32; // the server will cap this at our actual max. movement speed
			
			cmd.viewangles.pitch = 89f32;
			cmd.viewangles.yaw = (cmd.viewangles.yaw + 180f32) % 360f32; // flip us around
			cmd.viewangles.roll= 90f32; // apparently not capped, ggnore
		}
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}