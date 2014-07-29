use Cheat;
use GamePointers;
use sdk;
use sdk::Entity;
use libc;
use core::prelude::*;

pub struct CondRemover {
	enabled: bool,
}


impl Cheat for CondRemover {
	fn new() -> CondRemover {
		CondRemover { enabled: false}
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"CondRemover"
	}
	fn pre_createmove(&mut self, ptrs: &GamePointers, sequence_number: *mut libc::c_int,
			input_sample_frametime: *mut libc::c_float, active: *mut bool) {
		let mut me = sdk::utils::get_local_player_entity(ptrs);
		
		let myconds = me.mut_ptr_offset::<u32>(0x17AC + 0x048C);
		unsafe {
			
			*myconds = (*myconds) & ! (
				(1<<7) // taunting
				| (1<<0) // slow
				| (1 << 14)
				| (1 << 15)
			);
		}
		
		let mut wep: sdk::BaseCombatWeapon = unsafe {
			match ptrs.icliententitylist.get_client_entity_from_handle(*me.ptr_offset::<sdk::CBaseHandle>(0x0DA8)) {
				Some(wep) => Entity::from_ptr(wep),
				None => return // no active weapon
			}
		};
		if wep.get_classname() == "CTFMinigun" {
			unsafe {
				*wep.mut_ptr_offset(0x0B88) = 0i32; // state
			}
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}


