use Cheat;
use sdk;
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
		let icvar = ptrs.icvar.unwrap();
		let mut sv_cheats = icvar.find_var("sv_cheats").expect("sv_cheats cvar not found?");
		unsafe { sv_cheats.setvalue_raw(sdk::Int(1)); log!("sv_cheats set to 1 OK\n") };
		let mut bananer = icvar.find_var("tf_forced_holiday").expect("sv_cheats cvar not found?");
		unsafe { bananer.setvalue_raw(sdk::Int(1)); log!("happy bday set to 1 OK\n") };
	}
}