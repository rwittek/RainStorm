use Cheat;
use GamePointers;
use sdk;
use libc;
use core::prelude::*;
use core;
use sdk::BaseEntity;
use sdk::BaseCombatWeapon;

pub struct Crithack {
	enabled: bool,
	ctr: u32,
}


impl Cheat for Crithack {
	fn new() -> Crithack {
		Crithack { enabled: false, ctr: 0 }
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
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		let me = ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap();
		let wep: sdk::CombatWeapon = unsafe {
			match ptrs.icliententitylist.get_client_entity_from_handle(*me.ptr_offset::<sdk::CBaseHandle>(0x0DA8)) {
				Some(wep) => BaseEntity::from_ptr(wep),
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
