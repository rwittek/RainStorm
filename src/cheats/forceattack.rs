use {Cheat, GamePointers};
use sdk;

/// "Client-side checking should be fine."
/// - Valve Software
pub struct ForceAttack {
	enabled: bool
}

impl Cheat for ForceAttack {
	fn new() -> ForceAttack {
		ForceAttack { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"ForceAttack"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		use sdk::{IN_ATTACK};
		
		if !self.enabled {
			return;
		}
		
		if unsafe { sdk::ismousedown() } {
			cmd.buttons |= IN_ATTACK;
		}
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}