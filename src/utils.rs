use core::prelude::*;
use libc;
use sdk;
use core;

pub fn should_shoot(ivengineclient: &sdk::IVEngineClient, icliententitylist: &sdk::IClientEntityList,
		ienginetrace: &sdk::IEngineTrace, viewangles: &sdk::QAngle, hitbox: Option<i32>) -> bool {
	let mut trace = unsafe { sdk::trace_t::new() };
	//let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);

	let localplayer_entidx = ivengineclient.get_local_player();
	let local_baseentity = icliententitylist.get_client_entity(localplayer_entidx);
	
	let me: &mut sdk::C_BaseEntity = if local_baseentity.is_not_null() {
		unsafe { core::mem::transmute(local_baseentity) }
	} else {
		log!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); unsafe { libc::exit(1) }; 
	};

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

	ienginetrace.trace_ray(&ray, 0x46004001, None, &mut trace);
	
	if trace.base.allsolid  {
		return false;
	}

	if  trace.ent.is_not_null() {
		//log!("Hit hitbox {}, looking for {}\n",trace.hitbox, hitbox);
		let correct_location = match hitbox {
			Some(hb) => (trace.hitbox == (hb)),
			None => true
		};
		if correct_location && unsafe {
					(*trace.ent).get_classname() == "CTFPlayer" &&
					*((*trace.ent).ptr_offset::<u32>(0x00AC)) != *(me.ptr_offset(0x00AC))
				}
		{
			return true;
		}
		return false; //pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
	}

	false
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

pub fn map_all_players(entlist_ptr: *mut ::sdk::IClientEntityList, f: |*mut ::sdk::C_BaseEntity|) {
	match unsafe { entlist_ptr.to_option() } {
		Some(entlist) => {
			for idx in range(1i32, 32) {
				let maybe_ent_ptr = entlist.get_client_entity(idx);
				if maybe_ent_ptr.is_not_null() {
					let classname = unsafe {(*maybe_ent_ptr).get_classname()};
					//log!("classname: {}\n", classname);
					if classname == "CTFPlayer" {
						f(maybe_ent_ptr)
					}
				}
			}
		},
		None => ()
	}
}