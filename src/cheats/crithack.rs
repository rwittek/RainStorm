use Cheat;
use GamePointers;
use sdk;
use libc;
use core::prelude::*;
use core;

pub struct Crithack {
	enabled: bool,
	ismelee: bool // TODO: this is an ugly hack
}


impl Cheat for Crithack {
	fn new() -> Crithack {
		Crithack { enabled: false, ismelee: true }
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
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		let me = ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap();
		let wep = unsafe {
			match ptrs.icliententitylist.get_client_entity_from_handle(*me.ptr_offset::<sdk::CBaseHandle>(0x0DA8)) {
				Some(wep) => wep,
				None => return // no active weapon
			}
		};
		
		let mut try_cmdnum = cmd.command_number;
		if !sdk::utils::is_commandnum_critical(ptrs, wep, self.ismelee, try_cmdnum) {
			cmd.buttons = cmd.buttons & !sdk::IN_ATTACK;
		} else {
			log!("Crit bucket: {}\n", unsafe { sdk::raw::get_critbucket_contents(wep.get_ptr()) });
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
				
				
	fn set_config(&mut self, var: &str, val: &[&str]) {
		match var {
			"melee" => {
				self.ismelee = ::utils::str_to_integral::<uint>(val[0]) != 0u;
			}
			_ => {}
		}
	}

}
