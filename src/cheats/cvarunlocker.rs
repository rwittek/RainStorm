use core::prelude::*;
use Cheat;
use sdk;
use libc;
use CheatManager;
use GamePointers;

pub struct CvarUnlocker;

impl Cheat for CvarUnlocker {
	fn new() -> CvarUnlocker {
		CvarUnlocker
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Cvar Unlocker"
	}
	fn postinit(&mut self, ptrs: &GamePointers) {
		let icvar = unsafe { (ptrs.icvar.to_option().unwrap()) };
		let sv_cheats = icvar.find_var("sv_cheats");
		match sv_cheats {
			Some(cheats) => unsafe { (*cheats).setvalue_raw(sdk::Int(1)); log!("sv_cheats set to 1 OK\n") },
			None => {log!("No sv_cheats?!"); unsafe { libc::exit(1); }}
		}
	}
}