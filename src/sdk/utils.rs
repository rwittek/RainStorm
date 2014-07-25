use core::prelude::*;
use super::{get_tracefilter, IVEngineClient, IClientEntityList, IEngineTrace, trace_t, Ray_t, QAngle, C_BaseEntity, raw};

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
	direction = direction.scale( 8192.0f32 ) + eyes;
	
	let ray = Ray_t::new(&eyes, &direction);

	ienginetrace.trace_ray(&ray, 0x46004001, Some(unsafe { get_tracefilter(me) }), &mut trace);
	
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

impl  Iterator<C_BaseEntity> for EntityIterator {
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