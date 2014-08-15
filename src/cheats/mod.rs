use sdk;
use alloc::owned::Box;
use collections::Vec;
use core::prelude::*;
use alloc;
use core;
use GamePointers;
use libc;

pub mod triggerbot;
pub mod cvarunlocker;
pub mod namechanger;
pub mod aimbot;
pub mod nocmd;
pub mod nospread;
pub mod crithack;
pub mod bhop;
pub mod condremover;
pub mod spinbot;

pub static mut CHEAT_MANAGER: *mut CheatManager = 0 as *mut CheatManager;

pub fn cheatmgr_setup() {
	unsafe {
		log!("Allocating memory for CHEAT_MANAGER\n");
		CHEAT_MANAGER = alloc::heap::allocate(core::mem::size_of::<CheatManager>(), 8) as *mut CheatManager;
		log!("Constructing CHEAT_MANAGER\n");
		core::ptr::write(CHEAT_MANAGER, CheatManager::new());
		log!("Constructed O.K.\n");
	}
	
}

pub trait Cheat {
	fn new() -> Self;
	fn get_name<'a>(&'a self) -> &'a str;
	
	#[allow(unused_variable)]
	fn preinit(&mut self, ptrs: &GamePointers) {}
	#[allow(unused_variable)]
	fn postinit(&mut self, ptrs: &GamePointers) {}
	
	#[allow(unused_variable)]
	fn pre_createmove(&mut self, ptrs: &GamePointers, sequence_number: *mut libc::c_int,
			input_sample_frametime: *mut libc::c_float, active: *mut bool) {}		
	#[allow(unused_variable)]
	fn process_usercmd(&mut self, ptrs: &GamePointers, &mut sdk::CUserCmd) {}
	#[allow(unused_variable)]
	fn extramousesample(&mut self, ptrs: &GamePointers, input_sample_frametime: libc::c_float,
			active: bool) {}
			
	#[allow(unused_variable)]
	fn enable(&mut self) {}
	#[allow(unused_variable)]
	fn disable(&mut self) {}
	
	#[allow(unused_variable)]
	fn set_config(&mut self, var: &str, val: &[&str]) {}
}

pub struct CheatManager {
	cheats: Vec<Box<Cheat>>,
	
	ptrs: GamePointers
}



impl CheatManager {
	pub fn new() -> CheatManager {
		log!("Constructing cheats...\n");
		let triggerbot: Box<triggerbot::Triggerbot> = box Cheat::new();
		let cvarunlocker: Box<cvarunlocker::CvarUnlocker> = box Cheat::new();
		let namechanger: Box<namechanger::NameChanger> = box Cheat::new();
		let aimbot: Box<aimbot::Aimbot> = box Cheat::new();
		let nocmd: Box<nocmd::NoCmd> = box Cheat::new();
		let nospread: Box<nospread::NoSpread> = box Cheat::new();
		let crithack: Box<crithack::Crithack> = box Cheat::new();
		let bhop: Box<bhop::Bunnyhop> = box Cheat::new();
		let condremover: Box<condremover::CondRemover> = box Cheat::new();
		let spinbot: Box<spinbot::Spinbot> = box Cheat::new();
		log!("Creating CheatManager...\n");
		let mut mgr = CheatManager { 
			cheats: Vec::new(),
			
			ptrs: GamePointers::load()
		};
		
		log!("Pushing cheats...\n");
		mgr.cheats.push(nospread);
		mgr.cheats.push(crithack);
		mgr.cheats.push(cvarunlocker);
		mgr.cheats.push(aimbot);
		mgr.cheats.push(triggerbot);
		mgr.cheats.push(namechanger);
		mgr.cheats.push(nocmd);
		mgr.cheats.push(bhop);
		mgr.cheats.push(spinbot);
		mgr.cheats.push(condremover);
		mgr
	}
	pub fn handle_command(&mut self, command: &str, arguments: &[&str]) {
		log!("handling command {}\n", command);
		match command {
			"enable_cheat" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(cheat) => cheat.enable(),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			"disable_cheat" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(cheat) => cheat.disable(),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			"config" => {
				let cheat_name = arguments[0];
				match self.cheats.mut_iter().find(|cheat| cheat.get_name() == cheat_name) {
					Some(cheat) => cheat.set_config(arguments[1], arguments.slice_from(2)),
					None => log!("Could not find any cheats named {}\n", cheat_name) // cheat not found
				}
			},
			_ => {
				log!("Unrecognized command {}\n", {});
				// unrecognized
			}
		}
	}
	// Wrappers that run all the cheats' methods
	pub unsafe fn preinit(&mut self, appsysfactory: sdk::AppSysFactoryPtr) {
		self.ptrs.appsysfactory = Some(sdk::AppSysFactory::from_ptr(appsysfactory));
		
		self.ptrs.icvar = Some( sdk::get_icvar(&self.ptrs.appsysfactory.unwrap()) );
		
		for cheat in self.cheats.mut_iter() {
			cheat.preinit(&self.ptrs);
		}
	}
	pub fn postinit(&mut self) {
		for cheat in self.cheats.mut_iter() {
			cheat.postinit(&self.ptrs);
		}
	}
	pub fn pre_createmove(&mut self, sequence_number: *mut libc::c_int,
			input_sample_frametime: *mut libc::c_float, active: *mut bool) {
		for cheat in self.cheats.mut_iter() {
			cheat.pre_createmove(&self.ptrs, sequence_number, input_sample_frametime, active);
		}
	}
	pub fn process_usercmd(&mut self, cmd: &mut sdk::CUserCmd) {
		for cheat in self.cheats.mut_iter() {
			cheat.process_usercmd(&self.ptrs, cmd);
		}
	}
	pub fn extramousesample(&mut self, input_sample_frametime: libc::c_float, active: bool) {
		for cheat in self.cheats.mut_iter() {
			cheat.extramousesample(&self.ptrs, input_sample_frametime, active);
		}
	}
	
	pub fn get_gamepointers<'a>(&'a self) -> &'a GamePointers {
		&self.ptrs
	}
}	