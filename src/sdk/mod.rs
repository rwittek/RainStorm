pub use libc::c_char;
use libc;
use core;
use core::result::{Result, Ok, Err};
use core::option::{Option, None, Some};
use core::collections::Collection;
use core::raw::Repr;
use core::mem::transmute;
use core::ptr::RawPtr;

use CString;
// opaque phantom types
pub enum IVEngineClient {}
pub enum IBaseClientDLL {}
pub enum ConVar {}
pub enum ICvar {}
pub enum AppSysFactory {}
pub enum PhysicsFactory {}
pub enum Globals {}
pub enum CInput {}

pub struct QAngle {
	pub pitch: f32,
	pub yaw: f32,
	pub roll: f32
}

pub struct CUserCmd {
	vtable_ptr: *const i32,
	pub command_number: i32,
	pub tick_count: i32,
	
	pub viewangles: QAngle,  

	pub forwardmove: f32,
	pub sidemove: f32,
	pub upmove: f32,     
	pub buttons: i32,	
	// Impulse command issued.
	pub impulse: u8,   
	pub weaponselect: i32,	
	pub weaponsubtype: i32,

	random_seed: i32,

	pub mousedx: u16,
	pub mousedy: u16,

	pub hasbeenpredicted: bool
}

impl IVEngineClient {
	pub fn client_cmd(&mut self, command: &'static str) -> Result<(), &'static str> {
		let mut buf = [0u8, ..256];
		if command.len() >= buf.len() {
			return Err("Buffer overflow!");
		}
		unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(command.repr().data), command.len()); };
		buf[command.len()] = 0;
		unsafe { ivengineclient_clientcmd(self, unsafe { core::mem::transmute(buf.repr().data )}) };
		
		Ok(())
	}
	pub fn time(&mut self) -> f32 {
		unsafe { ivengineclient_time(self) } 
	}
}

pub enum ConVarValue {
	Int(libc::c_int),
	Str(CString)
}

impl ConVar {
	pub unsafe fn setvalue_raw(&mut self, val: ConVarValue) {
		match val {
			Int(v) => convar_setvalue_raw_int(self as *mut ConVar, v),
			Str(s) => convar_setvalue_str(self as *mut ConVar, s)
		}
	}
	pub unsafe fn setvalue(&mut self, val: ConVarValue) {
		match val {
			Int(v) => convar_setvalue_raw_int(self as *mut ConVar, v),
			Str(s) => convar_setvalue_str(self as *mut ConVar, s)
		}
	}
	pub unsafe fn freeze(&mut self) {
		convar_freeze(self as *mut ConVar)
	}
	pub unsafe fn clearflags(&mut self) {
		convar_clearflags(self as *mut ConVar)
	}
}

impl ICvar {
	pub fn find_var(&mut self, name: &str) -> Option<*mut ConVar> {
		let mut buf = [0u8, ..256];
		if name.len() >= buf.len() {
			return None
		} else {
			unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(name.repr().data), name.len()); }
			buf[name.len()] = 0;
			let raw_convar = unsafe { icvar_findvar(self as *mut ICvar, transmute(buf.repr().data)) };
			match raw_convar.is_null() {
				true => None,
				false => Some(raw_convar)
			}
		}
	}
}
	
extern "C" {
	pub fn getptr_ivengineclient() -> * mut IVEngineClient; // MAYBE NULL
	fn ivengineclient_clientcmd(engine: & mut IVEngineClient, cmd_string: * const c_char);
	fn ivengineclient_time(engine: &mut IVEngineClient) -> libc::c_float;
	
	pub fn getptr_ibaseclientdll() -> * mut IBaseClientDLL; // MAYBE NULL
	pub fn getptr_icvar(app_sys_factory: * mut AppSysFactory) -> * mut ICvar;
	
	pub fn getptr_cinput(client: *mut IBaseClientDLL) -> *mut CInput;
	fn icvar_findvar(icvar: * mut ICvar, name: * const char) -> * mut ConVar; // MAYBE NULL;
	pub fn convar_setvalue_raw_int(cvar: * mut ConVar, value: libc::c_int);
	pub fn convar_setvalue_str(cvar: * mut ConVar, value: CString);
	pub fn convar_clearflags(cvar: * mut ConVar);
	pub fn convar_freeze(cvar: * mut ConVar);
	
	pub fn trace_to_player(viewangles: &QAngle ) -> bool;
}

pub mod vmthook {
	use core;
	use log_print;
	use libc;
	use core::option::{Some, None, Option};
	use core::ptr::RawPtr;
	
	static VMT_MAX_SIZE_YOLO: u32 = 512; // swag
	type VMT = [*const (), ..VMT_MAX_SIZE_YOLO];
	
	pub struct VMTHooker { // this should be renamed.......
		original_vmt_ptr_ptr: *const VMT,
		original_vmt: VMT,
		patched_vmt_ptr: *mut VMT,
	}

	impl VMTHooker {
		pub unsafe fn new(vmt_ptr_ptr: *mut *const ()) -> VMTHooker {
			let vmt_ptr: *const VMT = core::mem::transmute(*vmt_ptr_ptr);
			// yes, we do leak this.
			// yolo.
			let new_vmt = libc::malloc(core::mem::size_of::<VMT>() as u32) as *mut VMT;
			log_print("Allocating new VMT\n");
			if new_vmt.is_null() {
				log_print("FAILED TO ALLOCATE VMT\n");
			}
			*new_vmt = *vmt_ptr; // christ this had better work
			let hooker = VMTHooker {
				original_vmt_ptr_ptr: core::mem::transmute(vmt_ptr_ptr),
				original_vmt: *vmt_ptr,
				patched_vmt_ptr: new_vmt
			};
			*vmt_ptr_ptr = (new_vmt) as *const VMT as *const ();
			hooker
		}
		
		pub unsafe fn hook(&mut self, method: uint, hook: *const ()) {
			(*(self.patched_vmt_ptr))[method] = hook;
		}
		
		pub unsafe fn get_orig_method(&self, method: uint) -> *const () {
			self.original_vmt[method]
		}
	}
}