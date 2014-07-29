use {Cheat, GamePointers};
use sdk;

pub struct NoCmd;

impl Cheat for NoCmd {
	fn new() -> NoCmd {
		NoCmd
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"NoCmd"
	}

	
	fn enable(&mut self) { unsafe { ::NOCMD_ENABLED = true; } }
	fn disable(&mut self) { unsafe { ::NOCMD_ENABLED = false; }}
}