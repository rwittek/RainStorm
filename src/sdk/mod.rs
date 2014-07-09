pub use libc::c_char;
use libc;
use core;
use core::result::{Result, Ok, Err};
use core::collections::Collection;
use core::raw::Repr;
use core::mem::transmute;

// opaque phantom types
pub enum IVEngineClient {}
pub enum IBaseClientDLL {}
pub enum ConVar {}
pub enum ICVar {}
pub enum AppSysFactory {}
pub enum PhysicsFactory {}
pub enum Globals {}

impl IVEngineClient {
	pub fn ClientCmd(&mut self, command: &'static str) -> Result<(), &'static str> {
		let mut buf = [0u8, ..256];
		if command.len() >= buf.len() {
			return Err("Buffer overflow!");
		}
		unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(command.repr().data), command.len()); };
		buf[command.len() + 1] = 0;
		unsafe { ivengineclient_clientcmd(self, unsafe { core::mem::transmute(buf.repr().data )}) };
		
		Ok(())
	}
}
#[link(name="wrapper", kind="static")]
extern {
	pub fn getptr_ivengineclient() -> * mut IVEngineClient; // MAYBE NULL
	fn ivengineclient_clientcmd(engine: & mut IVEngineClient, cmd_string: * const c_char);
	
	pub fn getptr_ibaseclientdll() -> * mut IBaseClientDLL; // MAYBE NULL
	pub fn getptr_icvar(app_sys_factory: * mut AppSysFactory) -> * mut ICVar;
	
	pub fn icvar_findvar(icvar: * mut ICVar, name: * const char) -> * mut ConVar; // MAYBE NULL;
	pub fn convar_setvalue_int(cvar: * mut ConVar, value: libc::c_int);
	pub fn convar_clearflags(cvar: * mut ConVar);
	pub static mut REAL_INIT: *const ();
}

pub mod vmthook {
	use core;
	use log_print;
	use libc;
	use core::option::{Some, None, Option};
	
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