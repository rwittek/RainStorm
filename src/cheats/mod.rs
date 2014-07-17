use sdk;
use alloc::owned::Box;
use collections::Vec;
use {Option, Some, None};
use core::iter::Iterator;
use core::prelude::*;
use alloc;
use core;
use GamePointers;

pub mod triggerbot;
pub mod speedhack;
pub mod cvarunlocker;
pub mod namechanger;

pub static mut CHEAT_MANAGER: *mut CheatManager = 0 as *mut CheatManager;

pub fn cheatmgr_setup() {
	unsafe {
		log!("Allocating memory for CHEAT_MANAGER\n");
		CHEAT_MANAGER = alloc::heap::allocate(core::mem::size_of::<CheatManager>(), 8) as *mut CheatManager;
		log!("Constructing CHEAT_MANAGER\n");
		core::ptr::write(CHEAT_MANAGER, CheatManager::new());
	}
	
}

pub trait Cheat {
	fn new() -> Self;
	fn get_name<'a>(&'a self) -> &'a str;
	
	fn preinit(&mut self, ptrs: &GamePointers) {}
	fn postinit(&mut self, ptrs: &GamePointers) {}
	fn process_usercmd(&mut self, ptrs: &GamePointers, &mut sdk::CUserCmd) {}
	
	fn enable(&mut self) {}
	fn disable(&mut self) {}
	
	fn set_config(&mut self, var: &str, val: &[&str]) {}
}

pub struct CheatManager {
	cheats: Vec<Box<Cheat>>,
	
	ptrs: GamePointers
}



impl CheatManager {
	pub fn new() -> CheatManager {
		let triggerbot: Box<triggerbot::Triggerbot> = box Cheat::new();
		let cvarunlocker: Box<cvarunlocker::CvarUnlocker> = box Cheat::new();
		let speedhack: Box<speedhack::Speedhack> = box Cheat::new();
		let namechanger: Box<namechanger::NameChanger> = box Cheat::new();
		
		let mut mgr = CheatManager { 
			cheats: Vec::new(),
			
			ptrs: GamePointers::load()
		};

		mgr.cheats.push(cvarunlocker);
		mgr.cheats.push(triggerbot);
		mgr.cheats.push(speedhack);
		mgr.cheats.push(namechanger);
		mgr
	}
	pub fn handle_command(&mut self, command: &str, arguments: &[&str]) {
		log!("handling command {}\n", command);
		match command {
			"enable_cheat" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(mut cheat) => cheat.enable(),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			"disable_cheat" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(mut cheat) => cheat.disable(),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			"config" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(mut cheat) => cheat.set_config(arguments[1], arguments.slice_from(2)),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			_ => {
				log!("Unrecognized command {}\n", {});
				// unrecognized
			}
		}
	}
	// Wrappers that run all the cheats' methods
	pub fn preinit(&mut self, appsysfactory: *mut sdk::AppSysFactory) {
		self.ptrs.appsysfactory = appsysfactory;
		
		let icvar_ptr = unsafe { sdk::getptr_icvar(appsysfactory) };
		if icvar_ptr.is_not_null() {
			self.ptrs.icvar = icvar_ptr;
		} else {
			quit!("ICvar null?\n");
		}
		
		for cheat in self.cheats.mut_iter() {
			cheat.preinit(&self.ptrs);
		}
	}
	pub fn postinit(&mut self) {
		for cheat in self.cheats.mut_iter() {
			cheat.postinit(&self.ptrs);
		}
	}
	pub fn process_usercmd(&mut self, cmd: &mut sdk::CUserCmd) {
		for cheat in self.cheats.mut_iter() {
			cheat.process_usercmd(&self.ptrs, cmd);
		}
	}
	
	pub fn get_gamepointers<'a>(&'a self) -> &'a GamePointers {
		&self.ptrs
	}
	
	
	// these getters should really use mutexes, but w/e
	// they don't return an option, because we validate as they are going in
	pub fn get_ivengineclient<'a>(&'a self) -> &'a mut sdk::IVEngineClient {
		if self.ptrs.ivengineclient.is_not_null() {
			unsafe { core::mem::transmute(self.ptrs.ivengineclient) }
		} else {
			quit!("IVEngineClient was null (this should never happen!)\n");
		}
	}
}	