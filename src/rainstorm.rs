#![no_std]
#![feature(intrinsics, lang_items, globs)]

extern crate libc;
extern crate core;

use core::prelude::*;
use core::mem;
use core::raw::Slice;

mod sdk;

#[no_mangle]
pub extern "C" fn rainstorm_entrypt() {
	unsafe { libc::puts(transmute::<&'static str, Slice>("hi\0").data); };
}


extern "rust-intrinsic" {
    fn transmute<T, U> (e: T) -> U;
}

struct Slice {
    data: *const i8,
    _len: uint,
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "begin_unwind"]
extern fn begin_unwind(args: &core::fmt::Arguments,
                       file: &str,
                       line: uint) -> ! {
    unsafe { core::intrinsics::abort(); }
}
