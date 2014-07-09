#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;
use core::raw::Repr;

mod sdk;
mod win32;

static mut IVENGINECLIENT_PTR: Option<*mut sdk::IVEngineClient> = None;
static mut IBASECLIENTDLL_PTR: Option<*mut sdk::IBaseClientDLL> = None;
static mut APPSYSFACTORY_PTR: Option<*mut sdk::AppSysFactory> = None;
static mut ICVAR_PTR: Option<*mut sdk::ICvar> = None;

#[no_mangle]
extern "stdcall" {
	fn Sleep(time: u32);
}

pub fn log_print(msg: &str) {
	unsafe { libc::write(LOG_FD, unsafe { core::mem::transmute(msg.repr().data) }, msg.repr().len as u32); };
}


static mut LOG_FD: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn rainstorm_inithook(app_sys_factory: *mut sdk::AppSysFactory, physics_factory: *mut sdk::PhysicsFactory, globals: *mut sdk::Globals) {
		APPSYSFACTORY_PTR = Some(app_sys_factory);
		ICVAR_PTR = Some(sdk::getptr_icvar(app_sys_factory));
		let sv_cheats = (*ICVAR_PTR.unwrap()).find_var("sv_cheats");
		match sv_cheats {
			Some(cheats) => sdk::convar_clearflags(cheats),
			None => log_print("No sv_cheats?!"),
		};
}
#[no_mangle]
pub extern "C" fn rainstorm_init(_log_fd: libc::c_int) {
	let log_fd = _log_fd;
	unsafe { LOG_FD = _log_fd; };
	let IVENGINECLIENT_PTR = Some(unsafe {
		let engine_ptr = sdk::getptr_ivengineclient();
		match engine_ptr.is_not_null() {
			true => { engine_ptr },
			false => { log_print("Engine not found, dying\n");
				libc::exit(1);
			}
		}
	});
	log_print("Engine found.\n");
	
	let ibaseclientdll: * mut sdk::IBaseClientDLL = unsafe {
		let ibaseclientdll_ptr = sdk::getptr_ibaseclientdll();
		match ibaseclientdll_ptr.is_not_null() {
			true => { ibaseclientdll_ptr },
			false => { log_print("IBaseClientDLL not found, dying\n");
				libc::exit(1);
			}
		}
	};
	log_print("IBaseClientDLL found.\n");

	unsafe {
	    
		let mut hooker = sdk::vmthook::VMTHooker::new(ibaseclientdll as *mut *const ());
		sdk::REAL_INIT = hooker.get_orig_method(0);
		log_print(
		if sdk::REAL_INIT.is_null() {
			"no init?!?"
		} else {
			"yay, init"
		});
		hooker.hook(0, core::mem::transmute(_Z22hooked_init_trampolinePFPvPKcPiES4_P15CGlobalVarsBase));
		//engine.ClientCmd("say hello world");
	};
	log_print("Hook installed... let's see!");
		
}

extern "stdcall" {
	// lol
	fn _Z22hooked_init_trampolinePFPvPKcPiES4_P15CGlobalVarsBase(app_sys_factory: *mut (), physics_factory: *mut (), globals: *mut ());
}
fn panic(msg: &str) -> ! {
	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "begin_unwind"] extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	loop {}
}