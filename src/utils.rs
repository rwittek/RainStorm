use core::prelude::*;
use libc;

pub struct Box<T> {
	ptr: *mut T
}

impl Box<T> {
	pub fn new() -> Box<T> {
		allocation = unsafe { libc::malloc(core::mem::size_of::<T>()) };
		if allocation.is_null() {
			format_args!(log, "Could not allocate {} bytes for Box, must die. rest in peperonis", core::mem::size_of(T));
			unsafe { libc::exit(1); }
		} else {
			Box { ptr: allocation };
		}
	}
}
impl<T> Deref<T> for Box<T> {
	pub fn deref<'a>(&'a self) -> &'a T {
		core::mem::transmute(self.ptr)
	}
}
impl<T> DerefMut<T> for Box<T> {
	pub fn deref_mut<'a>(&'a mut self) -> &'a mut T {
		core::mem::transmute(self.ptr)
	}
}
impl Drop for Box<T> {
	pub fn drop(&mut self) {
		unsafe { libc::free(self.ptr) };
	}
}
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