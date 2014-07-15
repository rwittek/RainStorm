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
	
	ivengineclient_ptr: *mut sdk::IVEngineClient,
	icliententitylist_ptr: *mut sdk::IClientEntityList,
	ibaseclientdll_ptr: *mut sdk::IBaseClientDLL,
	ienginetrace_ptr: *mut sdk::IEngineTrace,
	appsysfactory_ptr : *mut sdk::AppSysFactory,
	icvar_ptr: *mut sdk::ICvar,
}

impl CheatManager {
	pub fn new() -> CheatManager {
		let triggerbot: Box<triggerbot::Triggerbot> = box Cheat::new();
		let cvarunlocker: Box<cvarunlocker::CvarUnlocker> = box Cheat::new();
		
		let mut mgr = CheatManager { 
			cheats: Vec::new(),
			
			ivengineclient_ptr = unsafe {
				let engine_ptr = sdk::getptr_ivengineclient();
				match engine_ptr.is_not_null() {
					true => { log!("Engine found at {}.\n", engine_ptr); engine_ptr },
					false => { quit!("Engine not found, dying\n") }
				}
			},
			ibaseclientdll_ptr = unsafe { 
				let ibaseclientdll_ptr = sdk::getptr_ibaseclientdll();
				match ibaseclientdll_ptr.is_not_null() {
					true => { log!("IBaseClientDLL found at {}\n", ibaseclientdll_ptr); ibaseclientdll_ptr },
					false => { quit!("IBaseClientDLL not found, dying\n") }
				}
			},
			icliententitylist_ptr = unsafe {
				let icliententitylist_ptr = sdk::getptr_icliententitylist();
				match icliententitylist_ptr.is_not_null() {
					true => { log!("IClientEntityList found at {}\n", icliententitylist_ptr); icliententitylist_ptr },
					false => { quit!("IClientEntityList not found, dying\n") }
				}
			},
			ienginetrace_ptr = unsafe {
				let ienginetrace_ptr = sdk::getptr_ienginetrace();
				match ienginetrace_ptr.is_not_null() {
					true => { log!("IEngineTrace found at {}\n", ienginetrace_ptr); ienginetrace_ptr },
					false => { quit!("IEngineTrace not found, dying\n") }
				}
			}
		}

		
		mgr.cheats.push(cvarunlocker);
		mgr
	}
	
	// Wrappers that run all the cheats' methods
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