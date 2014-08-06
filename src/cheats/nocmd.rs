use {Cheat, GamePointers};
use sdk;

pub struct NoCmd {
	enabled: bool
}

impl Cheat for NoCmd {
	fn new() -> NoCmd {
		NoCmd { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"NoCmd"
	}

	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if self.enabled {
			cmd.tick_count = 0xFFFF; // lol
		}
	}
	fn enable(&mut self) {  self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}