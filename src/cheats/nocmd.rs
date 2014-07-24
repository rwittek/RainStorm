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
	fn process_usercmd(&mut self, _ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}

		cmd.tick_count = 0;
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}