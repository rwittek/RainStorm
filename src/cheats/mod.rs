use sdk;
use alloc::owned::Box;
use collections::Vec;
use {Option, Some, None};
use core::iter::Iterator;
use alloc;
use core;

pub mod triggerbot;
pub mod speedhack;
pub mod cvarunlocker;

pub static mut CHEAT_MANAGER: *mut CheatManager = 0 as *mut CheatManager;

pub fn cheatmgr_setup() {
	unsafe {
		CHEAT_MANAGER = alloc::heap::allocate(core::mem::size_of::<CheatManager>(), 8) as *mut CheatManager;
		core::ptr::write(CHEAT_MANAGER, CheatManager::new());
	}
}

pub trait Cheat {
	fn new() -> Self;
	fn get_name<'a>(&'a self) -> &'a str;
	
	fn preinit(&mut self) {}
	fn postinit(&mut self) {}
	fn process_usercmd(&mut self, &mut sdk::CUserCmd) {}
}

pub struct CheatManager {
	cheats: Vec<Box<Cheat>>,
	
	ivengineclient_ptr: Option<*mut sdk::IVEngineClient>,
	icliententitylist_ptr: Option<*mut sdk::IClientEntityList>,
	ibaseclientdll_ptr: Option<*mut sdk::IBaseClientDLL>,
	ienginetrace_ptr: Option<*mut sdk::IEngineTrace>,
	appsysfactory_ptr : Option<*mut sdk::AppSysFactory>,
	icvar_ptr: Option<*mut sdk::ICvar>,
}

impl CheatManager {
	pub fn new() -> CheatManager {
		let triggerbot: Box<triggerbot::Triggerbot> = box Cheat::new();
		let cvarunlocker: Box<cvarunlocker::CvarUnlocker> = box Cheat::new();
		
		let mut mgr = CheatManager { 
			cheats: Vec::new(),
			ivengineclient_ptr: None,
			ibaseclientdll_ptr: None,
			ienginetrace_ptr: None,
			appsysfactory_ptr: None,
			icvar_ptr: None
		};
		mgr.cheats.push(cvarunlocker);
		mgr
	}
	
	pub fn preinit(&mut self) {
		for cheat in self.cheats.mut_iter() {
			cheat.preinit();
		}
	}
	pub fn postinit(&mut self) {
		for cheat in self.cheats.mut_iter() {
			cheat.postinit();
		}
	}
	pub fn process_usercmd(&mut self, cmd: &mut sdk::CUserCmd) {
		for cheat in self.cheats.mut_iter() {
			cheat.process_usercmd(cmd);
		}
	}
}	