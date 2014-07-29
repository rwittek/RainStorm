use Cheat;
use GamePointers;
use sdk;


pub struct NoSpread {
	enabled: bool,
	
	target_cmdnum: i32,
	target_seed: i32,
}


impl Cheat for NoSpread {
	fn new() -> NoSpread {
		NoSpread { enabled: false, target_cmdnum: 2076615043, target_seed: unsafe { sdk::calc_seed_from_command_number(2076615043) } }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"NoSpread"
	}
	fn process_usercmd(&mut self, _ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		
		cmd.command_number = self.target_cmdnum;
		cmd.random_seed = self.target_seed;
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
	fn set_config(&mut self, var: &str, val: &[&str]) {
		match var {
			"cmdnum" => {
				let target = ::utils::str_to_integral(val[0]);
				self.target_seed = unsafe { sdk::calc_seed_from_command_number(target) };
				self.target_cmdnum = target;
				/*log!("Brute-forcing for seed: {}\n", target);
				let mut try_cmdnum = 0i32;
				loop {
					let current_seed = unsafe { sdk::calc_seed_from_command_number(try_cmdnum) };
					//log!("Calculated seed {} from command_number {}...\n", current_seed, try_cmdnum);
					if current_seed == target {
						break
					};
					
					try_cmdnum += 1;
				}
				log!("Brute-forcing OK, found command number {}\n", try_cmdnum);
				self.target_cmdnum = try_cmdnum;
				*/
			},
			_ => {}
		}
	}
}
