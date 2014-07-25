use core::prelude::*;
use libc;
use sdk;
use core;

pub fn trace_to_entity(ivengineclient: &sdk::IVEngineClient, icliententitylist: &sdk::IClientEntityList,
		ienginetrace: &sdk::IEngineTrace, viewangles: &sdk::QAngle) -> Option<*mut sdk::C_BaseEntity> {
	let mut trace = unsafe { sdk::trace_t::new() };
	
	let localplayer_entidx = ivengineclient.get_local_player();
	let local_baseentity = icliententitylist.get_client_entity(localplayer_entidx);
		
	let me: &sdk::C_BaseEntity = if local_baseentity.is_not_null() {
		unsafe { core::mem::transmute(local_baseentity) }
	} else {
		quit!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); 
	};
	//let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);


	let mut direction = sdk::Vector::new();

	unsafe {
		sdk::angle_vectors(viewangles, &mut direction, core::ptr::mut_null(), core::ptr::mut_null());
	}
	let mut eyes = me.get_origin();
	
	unsafe {
		let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
		eyes.x += (eye_offsets)[0];
		eyes.y += (eye_offsets)[1];
		eyes.z += (eye_offsets)[2];
	}
	direction = direction.scale( 8192.0f32 ) + eyes;
	
	let ray = sdk::Ray_t::new(&eyes, &direction);

	ienginetrace.trace_ray(&ray, 0x46004001, Some(unsafe { &mut *sdk::get_tracefilter(me as *const sdk::C_BaseEntity) }), &mut trace);
	
	if trace.base.allsolid  {
		None
	} else if trace.ent.is_not_null() {
		Some(trace.ent)
	} else {
		None
	}
}

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

pub struct EntityIterator<'a> {
	entlist: &'a sdk::IClientEntityList,
	current_index: i32,
	stop_at: i32
}
impl<'a> EntityIterator<'a> {
	pub fn new<'a>(entlist: &'a ::sdk::IClientEntityList) -> EntityIterator<'a> {
		let max_entindex = entlist.get_highest_entity_index();
		//log!("max entindex: {}\n", max_entindex);
		EntityIterator {
			entlist: entlist,
			current_index: 0,
			stop_at: max_entindex
		}
	}
}

impl<'a> Iterator<*mut ::sdk::C_BaseEntity> for EntityIterator<'a> {
	fn next(&mut self) -> Option<*mut ::sdk::C_BaseEntity> {
		while (self.current_index <= self.stop_at) { 
			let maybe_ent = self.entlist.get_client_entity(self.current_index);
			self.current_index += 1;

			if maybe_ent.is_null() {
				continue
			} else {
				return Some(maybe_ent);
			}
		}
		// if we fell through here, we have reached the end
		// rest in peperonis
		None
	}
}