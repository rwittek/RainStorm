#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;
use core::raw::Repr;

mod sdk;
mod win32;
/*
impl<'a> core::fmt::FormatWriter for &'a[u8] {
	fn write(&mut self, bytes: &[u8]) -> Result<(), core::fmt::FormatError> {
		match bytes.len() >= self.len() {
			true => Err(core::fmt::WriteError),
			false => {
				for (idx, byte) in bytes.iter().enumerate() {
					self[idx] = byte;
				}
				self[bytes.len()] = 0; // null terminate
				Ok(())
			}
		}
	}
}
*/

#[no_mangle]
extern "stdcall" {
	fn Sleep(time: u32);
}

#[no_mangle]
static mut REAL_INIT: *const () = 0 as *const ();

pub fn log_print(msg: &str) {
	unsafe { libc::write(LOG_FD, unsafe { core::mem::transmute(msg.repr().data) }, msg.repr().len as u32); };
}

static mut BASECLIENTDLL_HOOKER: Option<sdk::vmthook::VMTHooker> = None;
static mut LOG_FD: libc::c_int = 0;

#[no_mangle]
pub extern "C" fn rainstorm_init(_log_fd: libc::c_int) {
	let log_fd = _log_fd;
	unsafe { LOG_FD = _log_fd; };
	let engine: * mut sdk::IVEngineClient = unsafe {
		let engine_ptr = sdk::getptr_ivengineclient();
		match engine_ptr.is_not_null() {
			true => { engine_ptr },
			false => { log_print("Engine not found, dying\n");
				libc::exit(1);
			}
		}
	};
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
	    
		sdk::vmthook::VMTHooker::new(ibaseclientdll as *mut *const ());
		//BASECLIENTDLL_HOOKER.unwrap().hook(0, core::mem::transmute(_Z22hooked_init_trampolinePFPvPKcPiES4_P15CGlobalVarsBase));
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