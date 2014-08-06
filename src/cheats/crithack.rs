use Cheat;
use GamePointers;
use sdk;
use core::prelude::*;
use sdk::Entity;
use sdk::utils;
use libc;
	
pub struct Crithack {
	enabled: bool,
}


impl Cheat for Crithack {
	fn new() -> Crithack {
		Crithack { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Crithack"
	}
	fn pre_createmove(&mut self, ptrs: &GamePointers, sequence_number: *mut libc::c_int,
			input_sample_frametime: *mut libc::c_float, active: *mut bool) {
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
	}/*
		let me = utils::get_local_player_entity(ptrs);
		
		let wep: sdk::BaseCombatWeapon = unsafe {
			match ptrs.icliententitylist.get_client_entity_from_handle(*me.ptr_offset::<sdk::CBaseHandle>(0x0DA8)) {
				Some(wep) => Entity::from_ptr(wep),
				None => return // no active weapon
			}
		};
		
		let critbucket = unsafe { *wep.ptr_offset::<f32>(0x09D8 - 12) };
		//log!("crit bucket: {}\n", critbucket);
		
		if (critbucket < 500.0) {
			//log!("crit bucket too low!\n");
			return
		}
		
		if sdk::utils::is_commandnum_critical(ptrs, wep, cmd.command_number) {
			cmd.buttons &= !sdk::IN_ATTACK;
		}
	}*/
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
				
				

}
