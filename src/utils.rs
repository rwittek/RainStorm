use core::prelude::*;
use libc;

pub unsafe fn search_memory(start: *const (), len: uint, pattern: &[u8]) -> Option<*const ()> {
	// BE WARY OF INT OVERFLOW
	let mut offset = 0u;
	while offset + (pattern.len() as uint) < len {
		log!("Offset: {}\n", offset);
		if libc::memcmp((start as uint + offset) as *const libc::c_void, pattern.as_ptr() as *const libc::c_void, pattern.len() as u32) == 0 {
			return Some((start as uint + offset) as *const ());
		}
		offset = offset + 1;
	}
	
	None
}