use core::prelude::*;
use Cheat;
use GamePointers;
use Vec;

use sdk;

pub struct ChatSpam {
	enabled: bool,
	
	messages: Vec<&'static str>,
	curr_msg: uint
}

impl Cheat for ChatSpam {
	fn new() -> ChatSpam {
		let mut msgs = Vec::new();
		msgs.push("say ◄ ▲ ► ▼ ◄▼ ◄ ▲ ► ▼ ◄ ▲ ► ▼");
		
		ChatSpam { enabled: false, messages: msgs, curr_msg: 0 }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"ChatSpam"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		
		ptrs.ivengineclient.client_cmd(*self.messages.get(self.curr_msg));
		self.curr_msg += 1;
		if self.curr_msg >= self.messages.len() {
			self.curr_msg = 0
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
	fn set_config(&mut self, var: &str, val: &[&str]) {
		//
	}
}

impl ChatSpam {
}