#![feature(macro_rules, intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;
extern crate alloc;
extern crate collections;

pub use core::prelude::*;
pub use core::result::{Result, Ok, Err};
pub use cheats::Cheat;
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


pub unsafe fn locate_cinput() -> Option<*mut sdk::CInput> {
	let start_addr = REAL_CREATEMOVE as *const ();
	let result = utils::search_memory(start_addr, 100, &[0x8Bu8, 0x0D]);
	
	match result {
		Some(ptr) => {
			let cinput_ptr_ptr = *(((ptr as uint) + 2) as *const *const *mut sdk::CInput);
			log!("CInput pointer found at {}\n", cinput_ptr_ptr);
			log!("CInput found at {}\n", *cinput_ptr_ptr);
			Some(*(cinput_ptr_ptr))
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
	// TODO: null check
	APPSYSFACTORY_PTR = app_sys_factory;
	ICVAR_PTR = sdk::getptr_icvar(app_sys_factory);

	if cheats::CHEAT_MANAGER.is_not_null() {
		(*cheats::CHEAT_MANAGER).preinit();
	} else {
		log!("Cheat manager not found!\n");
		libc::exit(1);
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

	 
	unsafe {
		let mut hooker = vmthook::VMTHooker::new(IBASECLIENTDLL_PTR as *mut *const ());
		REAL_INIT = hooker.get_orig_method(0);
		REAL_CREATEMOVE = hooker.get_orig_method(21);
		hooker.hook(0, hooked_init_trampoline);
		hooker.hook(21, hooked_createmove_trampoline);
		log!("Hooks installed.\n");
		//engine.ClientCmd("say hello world");
	};
	
	unsafe { CINPUT_PTR = locate_cinput().unwrap() };
		
	cheats::cheatmgr_setup();
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