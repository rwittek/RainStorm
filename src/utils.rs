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

pub fn str_to_integral<T: ::core::num::Int + ::core::num::FromPrimitive>(string: &str) -> T {
	let mut n: T = ::core::num::Zero::zero();
	
	for digit in string.chars()
			.map(|chr| ::core::char::to_digit(chr, 10))
			.take_while(|maybe_digit| maybe_digit.is_some())
			.map(|maybe_digit| maybe_digit.unwrap())
	{
		n = (n * ::core::num::FromPrimitive::from_u8(10).unwrap()) + ::core::num::FromPrimitive::from_uint(digit).unwrap();
	}
	
	n
}

pub fn map_all_players(entlist_ptr: *mut ::sdk::IClientEntityList, f: |*mut ::sdk::C_BaseEntity|) {
	match unsafe { entlist_ptr.to_option() } {
		Some(entlist) => {
			for idx in range(1i32, 32) {
				let maybe_ent_ptr = entlist.get_client_entity(idx);
				if maybe_ent_ptr.is_not_null() {
					f(maybe_ent_ptr)
				}
			}
		},
		None => ()
	}
}