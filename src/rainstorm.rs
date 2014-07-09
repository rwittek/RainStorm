#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;

mod sdk;
mod win32;

impl core::fmt::FormatWriter for &[u8] {
	fn write(&mut self, bytes: &[u8]) -> Result<(), FormatError> {
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
#[no_mangle]
pub extern "C" fn rainstorm_init(log_fd: libc::c_int) {
	loop{}
	
	let engine_ref: &mut sdk::ffi::Engine = unsafe {
		let engine_ptr = sdk::ffi::getptr_engine();
		match engine_ptr.to_option() {
			Some(engine_ref) => std::mem::transmute(engine_ref), // ewww
			None => panic("no engine?")
		}
	};
	let x: [u8, ..256] = [0, ..256];
	println!("Engine is at: {}", engine_ref as *mut sdk::ffi::Engine);
}

fn panic(msg: &str) {
	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}