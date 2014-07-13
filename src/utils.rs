use core::prelude::*;
use libc;

pub unsafe fn search_memory(start: *const (), len: int, pattern: &[u8]) -> Option<*const ()> {
	// BE WARY OF INT OVERFLOW
	let mut offset = 0i;
	while offset + (pattern.len() as int) < len {
		if libc::memcmp((start as int + offset) as *const libc::c_void, pattern.as_ptr() as *const libc::c_void, pattern.len() as u32) == 0 {
			return Some((start as int + offset) as *const ());
		}
		offset = offset + 1;
	}
	
	None
}