use core::prelude::*;
use super::{get_tracefilter, IVEngineClient, IClientEntityList, IEngineTrace, trace_t, Ray_t, QAngle, C_BaseEntity, C_BaseAnimating, raw};
use sdk;
use libc;

pub fn trace_to_entity(ivengineclient: IVEngineClient, icliententitylist: IClientEntityList,
		ienginetrace: IEngineTrace, viewangles: &QAngle) -> Option<C_BaseEntity> {
	let mut trace = unsafe { trace_t::new() };
	
	let localplayer_entidx = ivengineclient.get_local_player();
	let me = icliententitylist.get_client_entity(localplayer_entidx).unwrap();
	
	//let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);


	let direction = viewangles.to_vector();
	
	let mut eyes = me.get_origin();
	
	unsafe {
		let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
		eyes.x += (eye_offsets)[0];
		eyes.y += (eye_offsets)[1];
		eyes.z += (eye_offsets)[2];
	}
	let trace_direction = direction.scale( 8192.0f32 ) + eyes;
	
	let ray = Ray_t::new(&eyes, &trace_direction);

	ienginetrace.trace_ray(&ray, 0x4600400B, Some(unsafe { get_tracefilter(me) }), &mut trace);
	
	if trace.base.allsolid  {
		None
	} else if trace.ent.is_not_null() {
		Some( unsafe { C_BaseEntity::from_ptr(trace.ent) })
	} else {
		None
	}
}

/// Iterates through all entities.
pub struct EntityIterator {
	entlist: IClientEntityList,
	current_index: i32,
	stop_at: i32
}

impl EntityIterator {
	pub fn new(entlist: IClientEntityList) -> EntityIterator {
		let max_entindex = entlist.get_highest_entity_index();
		//log!("max entindex: {}\n", max_entindex);
		EntityIterator {
			entlist: entlist,
			current_index: 0,
			stop_at: max_entindex
		}
	}
}

impl Iterator<C_BaseEntity> for EntityIterator {
	fn next(&mut self) -> Option<C_BaseEntity> {
		while (self.current_index <= self.stop_at) { 
			let maybe_ent = self.entlist.get_client_entity(self.current_index);
			self.current_index += 1;

			match maybe_ent {
				Some(ent) => return Some(ent),
				None => continue
			}
		}
		// if we fell through here, we have reached the end
		// rest in peperonis
		None
	}
}

pub struct HitboxPositionIterator {
	ent: C_BaseAnimating,
	modelinfo: sdk::IVModelInfo,
	current_hitbox: libc::c_int,
	num_hitboxes: libc::c_int
}
impl HitboxPositionIterator {
	pub fn new(ent: C_BaseAnimating, modelinfo: sdk::IVModelInfo) -> HitboxPositionIterator {
		HitboxPositionIterator { ent: ent, modelinfo: modelinfo, current_hitbox: 0, num_hitboxes: ent.get_num_hitboxes(modelinfo) }
	}
}
impl Iterator<sdk::Vector> for HitboxPositionIterator {
	fn next(&mut self) -> Option<sdk::Vector> {
		if self.current_hitbox == self.num_hitboxes {
			None
		} else {
			let pos = self.ent.get_hitbox_position(self.modelinfo, self.current_hitbox);
			self.current_hitbox += 1;
			Some(pos)
		}
	}
}