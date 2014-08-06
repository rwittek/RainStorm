#![feature(macro_rules, intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;
extern crate alloc;
extern crate collections;
extern crate rand;

pub use core::prelude::*;
pub use core::result::{Result, Ok, Err};
pub use cheats::{Cheat, CheatManager};
pub use alloc::owned::Box;
pub use collections::Vec;
use core::raw::Repr;


mod logging;
pub mod sdk;
mod vmthook;
pub mod utils;
mod cheats;

mod std {
	pub use core::fmt; //lol
	pub use core::option;
	pub use core::num;
}

#[allow(dead_code)]
pub mod cmath {
    use libc::{c_float, c_int};

    #[link_name = "m"]
    extern {
        pub fn acosf(n: c_float) -> c_float;
        pub fn asinf(n: c_float) -> c_float;
        pub fn atanf(n: c_float) -> c_float;
        pub fn atan2f(a: c_float, b: c_float) -> c_float;
        pub fn cbrtf(n: c_float) -> c_float;
        pub fn coshf(n: c_float) -> c_float;
        pub fn erff(n: c_float) -> c_float;
        pub fn erfcf(n: c_float) -> c_float;
        pub fn expm1f(n: c_float) -> c_float;
        pub fn fdimf(a: c_float, b: c_float) -> c_float;
        pub fn frexpf(n: c_float, value: &mut c_int) -> c_float;
        pub fn fmaxf(a: c_float, b: c_float) -> c_float;
        pub fn fminf(a: c_float, b: c_float) -> c_float;
        pub fn fmodf(a: c_float, b: c_float) -> c_float;
        pub fn nextafterf(x: c_float, y: c_float) -> c_float;
        pub fn hypotf(x: c_float, y: c_float) -> c_float;
        pub fn ldexpf(x: c_float, n: c_int) -> c_float;
        pub fn logbf(n: c_float) -> c_float;
        pub fn log1pf(n: c_float) -> c_float;
        pub fn ilogbf(n: c_float) -> c_int;
        pub fn modff(n: c_float, iptr: &mut c_float) -> c_float;
        pub fn sinhf(n: c_float) -> c_float;
        pub fn tanf(n: c_float) -> c_float;
        pub fn tanhf(n: c_float) -> c_float;
        pub fn tgammaf(n: c_float) -> c_float;

        /*#[cfg(unix)]
        pub fn lgammaf_r(n: c_float, sign: &mut c_int) -> c_float;

        #[cfg(windows)]
        #[link_name="__lgammaf_r"]
        pub fn lgammaf_r(n: c_float, sign: &mut c_int) -> c_float;*/
    }
}
#[no_mangle]
pub static mut NOCMD_ENABLED: bool = false;

#[no_mangle]
pub static mut REAL_INIT: *const () = 0 as *const();
#[no_mangle]
pub static mut REAL_CREATEMOVE: *const () = 0 as *const ();
#[no_mangle]
pub static mut REAL_EXTRAMOUSESAMPLE: *const () = 0 as *const ();
#[no_mangle]
pub static mut REAL_SERVERCMDKEYVALUES: *const () = 0 as *const ();
#[no_mangle]
pub static mut REAL_NETCHANNEL_SENDDATAGRAM: *const () = 0 as *const ();
#[no_mangle]
pub static mut CINPUT_PTR: *mut sdk::CInput = 0 as *mut sdk::CInput;

struct CString(*const libc::c_char);

impl CString {
	pub fn new(src: &'static [u8]) -> Option<CString> {
		let slice = src.repr();
		if unsafe { *((slice.data as uint + (slice.len - 1)) as *const u8) == 0 } {
			Some(CString(slice.data as *const libc::c_char))
		} else {
			None
		}
	}
	pub unsafe fn new_raw(src: *const u8) -> CString {
		CString(src as *const libc::c_char)
	}
}

#[no_mangle]
pub extern "C" fn rainstorm_getivengineclient() -> sdk::raw::IVEngineClientPtr {
	unsafe { (*(cheats::CHEAT_MANAGER)).get_gamepointers().ivengineclient.get_ptr() }
}
pub struct GamePointers {
	ivengineclient: sdk::IVEngineClient,
	icliententitylist: sdk::IClientEntityList,
	ibaseclientdll: sdk::IBaseClientDLL,
	ienginetrace: sdk::IEngineTrace,
	appsysfactory: Option<sdk::AppSysFactory>,
	ivmodelinfo: sdk::IVModelInfo,
	icvar: Option<sdk::ICvar>,
	iuniformrandomstream: sdk::IUniformRandomStream
}

impl GamePointers {
	pub fn load() -> GamePointers {
		log!("Loading GamePointers...\n");
		GamePointers {
			ivengineclient: sdk::get_ivengineclient(),
			ibaseclientdll: sdk::get_ibaseclientdll(),
			icliententitylist: sdk::get_icliententitylist(),
			ienginetrace: sdk::get_ienginetrace(),
			ivmodelinfo: sdk::get_ivmodelinfo(),
			appsysfactory: None,
			icvar: None,
			iuniformrandomstream: sdk::get_iuniformrandomstream()
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
			let load_instruction_operand = ((ptr as uint) + 2) as *const *const *mut sdk::CInput;
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
pub unsafe extern "C" fn rainstorm_preinithook(app_sys_factory: sdk::AppSysFactoryPtr, _physics_factory: *mut (), _globals: *mut ()) {
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
pub unsafe extern "C" fn rainstorm_pre_createmove(sequence_number: *mut libc::c_int, input_sample_frametime: *mut libc::c_float, active: *mut bool) {
	if cheats::CHEAT_MANAGER.is_not_null() {
		(*cheats::CHEAT_MANAGER).pre_createmove(sequence_number, input_sample_frametime, active);
	} else {
		quit!("Cheat manager not found!\n");
	};
}

#[no_mangle]
pub unsafe extern "C" fn rainstorm_process_usercmd(cmd: &mut sdk::CUserCmd) {
	if cheats::CHEAT_MANAGER.is_not_null() {
		//maybe_hook_inetchannel((*cheats::CHEAT_MANAGER).get_gamepointers());
		(*cheats::CHEAT_MANAGER).process_usercmd(cmd);
	} else {
		quit!("Cheat manager not found!\n");
	};
}

#[no_mangle]
pub unsafe extern "C" fn rainstorm_extramousesample(input_sample_frametime: libc::c_float, active: bool) {
	if cheats::CHEAT_MANAGER.is_not_null() {
		(*cheats::CHEAT_MANAGER).extramousesample(input_sample_frametime, active);
	} else {
		quit!("Cheat manager not found!\n");
	};
}

#[no_mangle]
pub extern "C" fn rainstorm_command_cb(c_arguments: *const libc::c_char) {
	let arguments_str = unsafe { core::str::raw::c_str_to_static_slice(c_arguments) };
	log!("Command callback: {}\n", arguments_str);
	
	let mut parts_iter = arguments_str.split(' ');
	let command = parts_iter.next().expect("No command type specified!");
	let parts: collections::Vec<&str> = parts_iter.collect();
	
	unsafe {
		if cheats::CHEAT_MANAGER.is_not_null() {
			(*cheats::CHEAT_MANAGER).handle_command(command, parts.as_slice());
		}
	}
}

#[no_mangle]
pub extern "C" fn rainstorm_init(log_fd: libc::c_int, hooked_init_trampoline: *const (), hooked_createmove_trampoline: *const (),
		hooked_extramousesample_trampoline: *const ()) {
	unsafe { let _ = logging::set_fd(log_fd).unwrap(); }
	log!("Rainstorm starting up!\n");

	cheats::cheatmgr_setup();
	
	unsafe {
		let mut ibaseclientdll_hooker = vmthook::VMTHooker::new((*cheats::CHEAT_MANAGER).get_gamepointers().ibaseclientdll.get_ptr().to_uint() as *mut *const ());
		REAL_INIT = ibaseclientdll_hooker.get_orig_method(0);
		REAL_CREATEMOVE = ibaseclientdll_hooker.get_orig_method(21);
		REAL_EXTRAMOUSESAMPLE = ibaseclientdll_hooker.get_orig_method(22);
		
		ibaseclientdll_hooker.hook(0, hooked_init_trampoline);
		ibaseclientdll_hooker.hook(21, hooked_createmove_trampoline);
		ibaseclientdll_hooker.hook(22, hooked_extramousesample_trampoline);
		
		// let mut ivengineclient_hooker = vmthook::VMTHooker::new((*cheats::CHEAT_MANAGER).get_gamepointers().ivengineclient.get_ptr().to_uint() as *mut *const ());
		// REAL_SERVERCMDKEYVALUES = ivengineclient_hooker.get_orig_method(185);
		// ivengineclient_hooker.hook(185, sdk::raw::get_hooked_servercmdkeyvalues());
		

		CINPUT_PTR = locate_cinput().expect("Failed to locate CInput pointer (signature not found)");
		let mut hooker = vmthook::VMTHooker::new(CINPUT_PTR as *mut *const ());
		hooker.hook(8, sdk::get_hooked_getusercmd())
	};
}


/// If we haven't seen this INetChannel before, hook it.
fn maybe_hook_inetchannel(ptrs: &GamePointers) {
 	static mut LAST_NETCHANNEL: Option<sdk::raw::INetChannelPtr> = None;
 	
 	unsafe {
 		let inetchannel = sdk::raw::get_current_inetchannel(ptrs.ivengineclient.get_ptr());
		//log!("chan: {}\n", inetchannel.to_uint());
 		let is_new_channel = match LAST_NETCHANNEL {
 			Some(last) => { inetchannel != last },
 			None => true
 		};
 		LAST_NETCHANNEL = Some(inetchannel);
 		
 		if !is_new_channel {
 			//log!("Not patching old netchannel");
 			return;
 		}
 		
 		let mut hooker = vmthook::VMTHooker::new(inetchannel.to_uint() as *mut *const ());
 		REAL_NETCHANNEL_SENDDATAGRAM =  hooker.get_orig_method(46);
 		hooker.hook(46, ::sdk::raw::get_netchannel_senddatagram_trampoline().to_uint() as *const ());
 		log!("senddatagram: {}\n", hooker.get_orig_method(46));
		
 	};
 }
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "begin_unwind"]
extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	log!("Failed at line {} of {}!\n", line, file);
	let _ = logging::log_fmt(fmt).ok(); // if we fail here, god help us
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