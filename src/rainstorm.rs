#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;

mod sdk;
mod win32;

#[no_mangle]
pub extern "C" fn rainstorm_init(log_fd: libc::c_int) {
	loop{}
	
	rainstorm_setup();
}

fn rainstorm_setup() {
	
	let engine_ref: &mut sdk::ffi::Engine = unsafe {
		let engine_ptr = sdk::ffi::getptr_engine();
		match engine_ptr.to_option() {
			Some(engine_ref) => std::mem::transmute(engine_ref), // ewww
			None => panic("no engine?")
		}
	};
	println!("Engine is at: {}", engine_ref as *mut sdk::ffi::Engine);
}

fn panic(msg: &str) {
	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}