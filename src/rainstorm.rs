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
pub extern "C" fn rainstorm_init(log_fd: libc::c_int) {
	
	let engine_ref: &mut sdk::ffi::Engine = unsafe {
		let engine_ptr = sdk::ffi::getptr_engine();
		match engine_ptr.to_option() {
			Some(engine_ref) => unsafe { core::mem::transmute(engine_ref) } , // ewww
			None => panic("no engine?")
		}
	};
	unsafe { libc::write(log_fd, unsafe { core::mem::transmute("Engine found OK!".repr().data) }, 16); }
		unsafe {
			sdk::ffi::engine_clientcmd(engine_ref, core::mem::transmute(b"say yo what's up\0".repr().data));
		};
		
}

fn panic(msg: &str) -> ! {
	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "begin_unwind"] extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	loop {}
}