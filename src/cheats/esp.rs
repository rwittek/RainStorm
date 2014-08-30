use core::prelude::*;
use {Cheat, GamePointers};
use sdk;
use sdk::{utils, raw, Entity, TFPlayer, OnTeam};

pub struct ESP {
	enabled: bool
}

impl Cheat for ESP {
	fn new() -> ESP {
		ESP { enabled: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"ESP"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let me: TFPlayer = unsafe { Entity::from_ptr(utils::get_local_player_entity(ptrs)) };
		//log!("ESP is happening\n");
		for ent in utils::EntityIterator::new(ptrs.icliententitylist)
				.filter(|ent| ent.get_classname() == "CObjectSentrygun" || ent.get_classname() == "CTFPlayer"
				|| ent.get_classname() == "CObjectDispenser" || ent.get_classname() == "CObjectTeleporter") {
			let should = {
				ent.get_team() != me.get_team()
			};
			
			unsafe {raw::do_glow(ent.get_ptr(), should) }
		}
	}
	
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}