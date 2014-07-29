use Cheat;
use GamePointers;
use sdk;
use core::prelude::*;
use sdk::Entity;
use sdk::utils;

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
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		if cmd.buttons & sdk::IN_ATTACK == 0 {
			return;
		}
		// EXPERIMENTAL
/* 		if !self.ismelee {
			self.ctr += 1;
			if self.ctr > 2 {
				self.ctr = 0;
			} else {
				return;
			}
		}
		 */
		let me = utils::get_local_player_entity(ptrs);
		
		let wep: sdk::BaseCombatWeapon = unsafe {
			match ptrs.icliententitylist.get_client_entity_from_handle(*me.ptr_offset::<sdk::CBaseHandle>(0x0DA8)) {
				Some(wep) => Entity::from_ptr(wep),
				None => return // no active weapon
		}
		};
		
		let mut try_cmdnum = cmd.command_number;
		while !sdk::utils::is_commandnum_critical(ptrs, wep, try_cmdnum) {
			try_cmdnum = try_cmdnum + 1;
		}
		
		cmd.command_number = try_cmdnum;
		cmd.random_seed = unsafe { sdk::raw::calc_seed_from_command_number(try_cmdnum) };
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
				
				

}
