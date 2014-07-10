#![feature(intrinsics, lang_items, globs)]
#![no_std]

extern crate libc;
extern crate core;

use core::prelude::*;
use core::raw::Repr;

mod sdk;


static mut IVENGINECLIENT_PTR: Option<*mut sdk::IVEngineClient> = None;
static mut IBASECLIENTDLL_PTR: Option<*mut sdk::IBaseClientDLL> = None;
static mut APPSYSFACTORY_PTR: Option<*mut sdk::AppSysFactory> = None;
static mut ICVAR_PTR: Option<*mut sdk::ICvar> = None;

#[no_mangle]
extern "stdcall" {
	fn Sleep(time: u32);
}

struct CString(*const libc::c_char);

impl CString {
	pub fn new(src: &'static str) -> Option<CString> {
		let slice = src.repr();
		if unsafe { *((slice.data as uint + (slice.len - 1)) as *const u8) == 0 } {
			Some(CString(slice.data as *const libc::c_char))
		} else {
			None
		}
	}
}

pub fn log_print(msg: &str) {
	unsafe { libc::write(LOG_FD, unsafe { core::mem::transmute(msg.repr().data) }, msg.repr().len as u32); };
}


static mut LOG_FD: libc::c_int = 0;

pub unsafe fn locate_cinput() -> Option<*mut sdk::CInput> {
	let start_addr: u32 = sdk::REAL_CREATEMOVE as u32;
	let mut state = 0u8;
	for offset in core::iter::range(start_addr, start_addr + 100) {
		log_print("meow!");
		let b1 = offset as *const u8;
		let b2 = (offset + 1) as *const u8;
		if (*b1 == 0x8Bu8) && (*b2 == 0x0Du8) {
			return Some(**((offset + 2) as *mut *mut *mut sdk::CInput));
		}
	}
	log_print(":(");
	None
}
#[no_mangle]
pub unsafe extern "C" fn rainstorm_preinithook(app_sys_factory: *mut sdk::AppSysFactory, physics_factory: *mut sdk::PhysicsFactory, globals: *mut sdk::Globals) {
	log_print("Pre-init hook running...\n");
	APPSYSFACTORY_PTR = Some(app_sys_factory);
	ICVAR_PTR = Some(sdk::getptr_icvar(app_sys_factory));
}

#[no_mangle]
pub unsafe extern "C" fn rainstorm_postinithook() {
	log_print("Post-init hook running...\n");
	let sv_cheats = (*ICVAR_PTR.unwrap()).find_var("sv_cheats");
	match sv_cheats {
		Some(cheats) => { (*cheats).setvalue_raw(sdk::Int(1)); log_print("sv_cheats 1 OK\n") },
		None => log_print("No sv_cheats?!"),
	};
	let cvar_name = (*ICVAR_PTR.unwrap()).find_var("name");
	match cvar_name {
		Some(name) => {
			log_print("found name\n");
			//(*name).setvalue(sdk::Str(CString::new("lil' timmy\0").unwrap()));
			//(*name).freeze();
			//log_print("name changed\n");
		}, 
		None => log_print("no name?!\n")
	}
	let cvar_interp = (*ICVAR_PTR.unwrap()).find_var("cl_interp");
	match cvar_interp {
		Some(interp) => {
			log_print("found interp\n");
			//(*interp).clearflags();
			//(*interp).setvalue(sdk::Str(CString::new("20").unwrap()));
			//(*interp).freeze();
			log_print("interp changed\n");
		}, 
		None => log_print("no interp?!\n")
	}
}
#[no_mangle]
pub extern "C" fn rainstorm_process_usercmd(cmd: &mut sdk::CUserCmd) {
	cmd.forwardmove = 0f32;
}
#[no_mangle]
pub unsafe extern "C" fn rainstorm_getivengineclient() -> *mut sdk::IVEngineClient {
	match IVENGINECLIENT_PTR {
		Some(p) => p,
		None => core::ptr::mut_null()
	}
}
#[no_mangle]
pub extern "C" fn rainstorm_init(_log_fd: libc::c_int) {
	let log_fd = _log_fd;
	unsafe { LOG_FD = _log_fd; };
	unsafe { IVENGINECLIENT_PTR = Some({
		let engine_ptr = sdk::getptr_ivengineclient();
		match engine_ptr.is_not_null() {
			true => { engine_ptr },
			false => { log_print("Engine not found, dying\n");
				libc::exit(1);
			}
		}
	})};
	log_print("Engine found.\n");
	
	unsafe {IBASECLIENTDLL_PTR = Some({
		let ibaseclientdll_ptr = sdk::getptr_ibaseclientdll();
		match ibaseclientdll_ptr.is_not_null() {
			true => { ibaseclientdll_ptr },
			false => { log_print("IBaseClientDLL not found, dying\n");
				libc::exit(1);
			}
		}
	})};
	
	log_print("IBaseClientDLL found.\n");
	
	unsafe {
		let mut hooker = sdk::vmthook::VMTHooker::new(IBASECLIENTDLL_PTR.unwrap() as *mut *const ());
		sdk::REAL_INIT = hooker.get_orig_method(0);
		sdk::REAL_CREATEMOVE = hooker.get_orig_method(21);
		hooker.hook(0, core::mem::transmute(_Z22hooked_init_trampolinePFPvPKcPiES4_P15CGlobalVarsBase));
		hooker.hook(21, core::mem::transmute(_Z28hooked_createmove_trampolineifb));
		log_print("Init hook installed.\n");
		//engine.ClientCmd("say hello world");
	};
	
	unsafe { sdk::CINPUT_PTR = locate_cinput().unwrap() };
	
	log_print("CInput found.\n");

	log_print("Hook installed... let's see!");
		
}

extern "stdcall" {
	// lol
	fn _Z22hooked_init_trampolinePFPvPKcPiES4_P15CGlobalVarsBase(app_sys_factory: *mut (), physics_factory: *mut (), globals: *mut ());
	fn _Z28hooked_createmove_trampolineifb(sequence_number: libc::c_int, input_sample_frametime: libc::c_float, active: bool);
}
fn panic(msg: &str) -> ! {
	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "begin_unwind"] extern fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
	loop {}
}