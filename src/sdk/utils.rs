use core::prelude::*;
use GamePointers;
use super::{get_tracefilter, IClientEntityList, trace_t, Ray_t, QAngle,
	Entity, Animating, raw};
use sdk;
use libc;

pub fn get_local_player_entity(ptrs: &GamePointers) -> raw::C_BaseEntityPtr {
	let localplayer_entidx = ptrs.ivengineclient.get_local_player();
	ptrs.icliententitylist.get_client_entity(localplayer_entidx).expect("Local player entity not found!")
}
	
pub fn trace_to_entity(ptrs: &GamePointers, viewangles: &QAngle) -> Option<(raw::C_BaseEntityPtr, i32)> {
	let me = get_local_player_entity(ptrs);
	let mut trace = unsafe { trace_t::new() };
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

	ptrs.ienginetrace.trace_ray(&ray, 0x4600400B, Some(get_tracefilter(me)), &mut trace);
	
	if trace.base.allsolid  {
		None
	} else if trace.ent.is_not_null() {
		Some((trace.ent, trace.hitbox ))
	} else {
		None
	}
}

pub fn is_commandnum_critical<WepType: sdk::CombatWeapon>(ptrs: &GamePointers, weapon: WepType, commandnum: i32) -> bool {
	let index = match weapon.is_melee() {
		true => (weapon.get_index() << 16) | (ptrs.ivengineclient.get_local_player() << 8),
		false => (weapon.get_index() << 8) | ptrs.ivengineclient.get_local_player()
	};

	let global_seed = unsafe { raw::calc_seed_from_command_number(commandnum) };
	
	ptrs.iuniformrandomstream.set_seed(global_seed ^ index);
	
	ptrs.iuniformrandomstream.random_int(0, 9999) < match weapon.is_melee() { true => 1500, false => 50 }
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

impl Iterator<raw::C_BaseEntityPtr> for EntityIterator {
	fn next(&mut self) -> Option<raw::C_BaseEntityPtr> {
		while self.current_index <= self.stop_at { 
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

pub struct HitboxPositionIterator<EntType> {
	ent: EntType,
	modelinfo: sdk::IVModelInfo,
	current_hitbox: libc::c_int,
	num_hitboxes: libc::c_int
}
impl<EntType: Animating> HitboxPositionIterator<EntType> {
	pub fn new(ent: EntType, modelinfo: sdk::IVModelInfo) -> HitboxPositionIterator<EntType> {
		HitboxPositionIterator { ent: ent, modelinfo: modelinfo, current_hitbox: 0, num_hitboxes: ent.get_num_hitboxes(modelinfo) }
	}
}
impl<EntType: Animating> Iterator<sdk::Vector> for HitboxPositionIterator<EntType> {
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