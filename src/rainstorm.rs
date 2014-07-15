#![feature(macro_rules, intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;
extern crate alloc;
extern crate collections;

pub use core::prelude::*;
pub use core::result::{Result, Ok, Err};
pub use cheats::{Cheat, CheatManager};
pub use alloc::owned::Box;
use core::raw::Repr;


pub mod sdk;
mod logging;
mod vmthook;
mod utils;
mod cheats;

mod std {
	pub use core::fmt; //lol
}


#[no_mangle]
pub static mut REAL_INIT: *const () = 0 as *const();
#[no_mangle]
pub static mut REAL_CREATEMOVE: *const () = 0 as *const ();
#[no_mangle]
pub static mut CINPUT_PTR: *mut sdk::CInput = 0 as *mut sdk::CInput;

struct CString(*const libc::c_char);

impl CString {
	pub fn new(src: &'static str) -> Option<CString> {
		let slice = src.repr();
		if unsafe { *((slice.data as uint + (slice.len - 1)) as *const u8) == 0 } {
			Some(CString(slice.data as *const libc::c_char))
		} else {
			None
		}
	}
}

#[no_mangle]
pub extern "C" fn rainstorm_getivengineclient() -> *mut sdk::IVEngineClient {
	unsafe { (*(cheats::CHEAT_MANAGER)).get_gamepointers().ivengineclient }
}
pub struct GamePointers {
	ivengineclient: *mut sdk::IVEngineClient,
	icliententitylist: *mut sdk::IClientEntityList,
	ibaseclientdll: *mut sdk::IBaseClientDLL,
	ienginetrace: *mut sdk::IEngineTrace,
	appsysfactory: *mut sdk::AppSysFactory,
	icvar: *mut sdk::ICvar
}

impl GamePointers {
	pub fn load() -> GamePointers {
		GamePointers {
			ivengineclient: unsafe {
				let engine_ptr = sdk::getptr_ivengineclient();
				match engine_ptr.is_not_null() {
					true => { log!("Engine found at {}.\n", engine_ptr); engine_ptr },
					false => { quit!("Engine not found, dying\n") }
				}
			},
			ibaseclientdll: unsafe { 
				let ibaseclientdll_ptr = sdk::getptr_ibaseclientdll();
				match ibaseclientdll_ptr.is_not_null() {
					true => { log!("IBaseClientDLL found at {}\n", ibaseclientdll_ptr); ibaseclientdll_ptr },
					false => { quit!("IBaseClientDLL not found, dying\n") }
				}
			},
			icliententitylist: unsafe {
				let icliententitylist_ptr = sdk::getptr_icliententitylist();
				match icliententitylist_ptr.is_not_null() {
					true => { log!("IClientEntityList found at {}\n", icliententitylist_ptr); icliententitylist_ptr },
					false => { quit!("IClientEntityList not found, dying\n") }
				}
			},
			ienginetrace: unsafe {
				let ienginetrace_ptr = sdk::getptr_ienginetrace();
				match ienginetrace_ptr.is_not_null() {
					true => { log!("IEngineTrace found at {}\n", ienginetrace_ptr); ienginetrace_ptr },
					false => { quit!("IEngineTrace not found, dying\n") }
				}
			},
			appsysfactory: core::ptr::mut_null(),
			icvar: core::ptr::mut_null()
		}
	}
}


pub unsafe fn locate_cinput() -> Option<*mut sdk::CInput> {
	let start_addr = REAL_CREATEMOVE as *const ();
	log!("Locating CInput from CreateMove at {}\n", start_addr);
	let result = utils::search_memory(start_addr, 100, &[0x8B, 0x0D]);
	//let result = utils::search_memory(((result1 as uint) + 2) as *const (), 100, &[0x8B, 0x0D]);
	match result {
		Some(ptr) => {
			let load_instruction_operand = (((ptr as uint) + 2) as *const *const *mut sdk::CInput);
			log!("CInput load found at {}\n", load_instruction_operand); 
			let cinput_ptr_ptr = *load_instruction_operand;
			log!("CInput pointer: {}\n", cinput_ptr_ptr);
			log!("CInput found at {}\n", *cinput_ptr_ptr);
			Some((*cinput_ptr_ptr))
		},
		None => {
			log!("CInput not found?!?\n");
			None
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn rainstorm_preinithook(app_sys_factory: *mut sdk::AppSysFactory, _physics_factory: *mut sdk::PhysicsFactory, _globals: *mut sdk::Globals) {
	log!("pre-init hook running\n");

	if cheats::CHEAT_MANAGER.is_not_null() {
		(*cheats::CHEAT_MANAGER).preinit(app_sys_factory);
	} else {
		quit!("Cheat manager not found!\n");
	};
}

#[no_mangle]
pub unsafe extern "C" fn rainstorm_postinithook() {
	log!("Post-init hook running...\n");
	if cheats::CHEAT_MANAGER.is_not_null() {
		(*cheats::CHEAT_MANAGER).postinit();
	} else {
		log!("Cheat manager not found!\n");
		libc::exit(1);
	};
}

#[no_mangle]
pub extern "C" fn rainstorm_process_usercmd(cmd: &mut sdk::CUserCmd) {
	unsafe {
		if cheats::CHEAT_MANAGER.is_not_null() {
			(*cheats::CHEAT_MANAGER).process_usercmd(cmd);
		} else {
			log!("Cheat manager not found!\n");
			libc::exit(1);
		};
	}
}

#[no_mangle]
pub extern "C" fn rainstorm_init(log_fd: libc::c_int, hooked_init_trampoline: *const (), hooked_createmove_trampoline: *const ()) {
	unsafe { let _ = logging::set_fd(log_fd).ok().unwrap(); }
	log!("Rainstorm starting up!");

	cheats::cheatmgr_setup();
	
	unsafe {
		let mut ibaseclientdll_hooker = vmthook::VMTHooker::new((*cheats::CHEAT_MANAGER).get_gamepointers().ibaseclientdll as *mut *const ());
		REAL_INIT = ibaseclientdll_hooker.get_orig_method(0);
		REAL_CREATEMOVE = ibaseclientdll_hooker.get_orig_method(21);
		
		ibaseclientdll_hooker.hook(0, hooked_init_trampoline);
		ibaseclientdll_hooker.hook(21, hooked_createmove_trampoline);
	}
	
	unsafe { CINPUT_PTR = locate_cinput().unwrap() };
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "begin_unwind"]
extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	log!("Failed at line {} of {}!\n", line, file);
	let _ = logging::log_fmt(fmt).ok().unwrap();
	unsafe { libc::exit(42); }
}

#[allow(non_snake_case_functions)]
#[no_mangle]
pub extern "C" fn _imp___onexit() {
}
#[no_mangle]
pub extern "C" fn __dllonexit() {
}
#[no_mangle]
pub extern "C" fn __setusermatherr() {
	
}