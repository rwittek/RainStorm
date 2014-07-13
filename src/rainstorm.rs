#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;
use core::raw::Repr;
use logging::{log};

mod sdk;


static mut IVENGINECLIENT_PTR: *mut sdk::IVEngineClient = 0;
static mut IBASECLIENTDLL_PTR: *mut sdk::IBaseClientDLL = 0;
static mut APPSYSFACTORY_PTR: *mut sdk::AppSysFactory = 0;
static mut ICVAR_PTR: Option<*mut sdk::ICvar> = 0;
#[no_mangle]
pub static mut REAL_INIT: *const () = 0 as *const();
#[no_mangle]
pub static mut REAL_CREATEMOVE: *const () = 0 as *const ();
#[no_mangle]
pub static mut CINPUT_PTR: *mut sdk::CInput = 0 as *mut sdk::CInput;
#[no_mangle]
extern "stdcall" {
	fn Sleep(time: u32);
}

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
	let result = utils::search_memory(startaddr, 100, &[0x8Bu8, 0x0D]);
	
	match Result {
		Some(cinput_ptr) => { 
			format_args!(log, "CInput found at {}\n", cinput_ptr);
			cinput_ptr
		},
		None {
			format_args!(log, "CInput not found?!?\n", cinput_ptr);
			None
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn rainstorm_preinithook(app_sys_factory: *mut sdk::AppSysFactory, physics_factory: *mut sdk::PhysicsFactory, globals: *mut sdk::Globals) {
	format_args!(log, "pre-inint hook running\n");
	APPSYSFACTORY_PTR = Some(app_sys_factory);
	ICVAR_PTR = Some(sdk::getptr_icvar(app_sys_factory));
}

#[no_mangle]
pub unsafe extern "C" fn rainstorm_postinithook() {
	format_args!(log, "Post-init hook running...\n");
	//let sv_cheats = (*ICVAR_PTR.unwrap()).find_var("sv_cheats");
	//match sv_cheats {
	//	Some(cheats) => { (*cheats).setvalue_raw(sdk::Int(1)); log_print("sv_cheats 1 OK\n") },
	//	None => log_print("No sv_cheats?!"),
	//};
	//let cvar_name = (*ICVAR_PTR.unwrap()).find_var("name");
	//match cvar_name {
	//	Some(name) => {
	//		log_print("found name\n");
			//(*name).setvalue(sdk::Str(CString::new("lil' timmy\0").unwrap()));
			//(*name).freeze();
			//log_print("name changed\n");
	//	}, 
	//	None => log_print("no name?!\n")
	//}
	//let cvar_interp = (*ICVAR_PTR.unwrap()).find_var("cl_interp");
	//match cvar_interp {
	//	Some(interp) => {
	//		log_print("found interp\n");
	//		//(*interp).clearflags();
	//		//(*interp).setvalue(sdk::Str(CString::new("20").unwrap()));
	//		//(*interp).freeze();
	//		log_print("interp changed\n");
	//	}, 
	//	None => log_print("no interp?!\n")
	//}
}
static mut LAST_TTP: u32 = 0;
#[no_mangle]
pub extern "C" fn rainstorm_process_usercmd(cmd: &mut sdk::CUserCmd) {
	if  (cmd.buttons & 1 == 1) {
		cmd.buttons = !((!cmd.buttons) | 1);
		unsafe { if  sdk::trace_to_player(&cmd.viewangles) {
			cmd.buttons = cmd.buttons | 1;;
		}};
	}
	if  false { // (cmd.buttons & (1 << 0)) == 0 && (cmd.forwardmove > 0.1f32 || cmd.forwardmove < -0.001f32) {
		// speedhack time
		let x = cmd.forwardmove;
		cmd.forwardmove = -999f32;
		//cmd.sidemove = 1.0f32 * x;
		
		cmd.viewangles.pitch = 89f32;
		cmd.viewangles.yaw = ((cmd.viewangles.yaw + 180f32) % 360f32);
		cmd.viewangles.roll= 49f32;
	}
}
#[no_mangle]
pub unsafe extern "C" fn rainstorm_getivengineclient() -> *mut sdk::IVEngineClient {
	match IVENGINECLIENT_PTR {
		Some(p) => p,
		None => core::ptr::mut_null()
	}
}
#[no_mangle]
pub extern "C" fn rainstorm_init(_log_fd: libc::c_int, hooked_init_trampoline: *const (), hooked_createmove_trampoline: *const ()) {
	let log_fd = _log_fd;
	unsafe { LOG_FD = _log_fd; };
	unsafe { IVENGINECLIENT_PTR = {
		let engine_ptr = sdk::getptr_ivengineclient();
		match engine_ptr.is_not_null() {
			true => { format_args!(log, "Engine found at {}.\n", engine_ptr); engine_ptr },
			false => { format_args!(log, "Engine not found, dying\n");
				libc::exit(1);
			}
		}
	}};
	format_args!(log, "Engine found at {}.\n", IVENGINECLIENT_PTR);
	
	unsafe {IBASECLIENTDLL_PTR = Some({
		let ibaseclientdll_ptr = sdk::getptr_ibaseclientdll();
		match ibaseclientdll_ptr.is_not_null() {
			true => { format_args!(log, "IBaseClientDLL found at {}\n", ibaseclientdll_ptr); ibaseclientdll_ptr },
			false => { format_args!(log, "IBaseClientDLL not found, dying\n");
				libc::exit(1);
			}
		}
	})};
	
	unsafe {
		let mut hooker = sdk::vmthook::VMTHooker::new(IBASECLIENTDLL_PTR.unwrap() as *mut *const ());
		REAL_INIT = hooker.get_orig_method(0);
		REAL_CREATEMOVE = hooker.get_orig_method(21);
		hooker.hook(0, hooked_init_trampoline);
		hooker.hook(21, hooked_createmove_trampoline);
		format_args!(log, "Init hook installed.\n");
		//engine.ClientCmd("say hello world");
	};
	
	unsafe { CINPUT_PTR = locate_cinput().unwrap() };
		
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "begin_unwind"] extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _imp___onexit() {
}
#[no_mangle]
pub extern "C" fn __dllonexit() {
}
#[no_mangle]
pub extern "C" fn __setusermatherr() {
	
}