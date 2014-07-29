use Cheat;
use GamePointers;
use sdk;
use sdk::Entity;

pub struct Bunnyhop {
	enabled: bool,
	
	wasinair: bool
}


impl Cheat for Bunnyhop {
	fn new() -> Bunnyhop {
		Bunnyhop { enabled: false, wasinair: false }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Bunnyhop"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		let me = ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap();
		let flags: i32 = unsafe { *me.ptr_offset(0x0378) };
		
		if (flags & 1) == 0 && self.wasinair { // FL_ONGROUND
			cmd.buttons &= !sdk::IN_JUMP;
			cmd.upmove = 320.0;
		}
		self.wasinair = (flags & 1) == 0;
		
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}
