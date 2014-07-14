pub use libc::c_char;
use libc;
use core;
use core::result::{Result, Ok, Err};
use core::option::{Option, None, Some};
use core::collections::Collection;
use core::raw::Repr;
use core::mem::transmute;
use core::ptr::RawPtr;

pub use CString;
// opaque phantom types
pub enum IVEngineClient {}
pub enum IBaseClientDLL {}
pub enum ConVar {}
pub enum ICvar {}
pub enum AppSysFactory {}
pub enum PhysicsFactory {}
pub enum Globals {}
pub enum CInput {}
pub enum C_BaseEntity {}
pub enum IHandleEntity {}
pub enum IClientEntityList {}
pub enum IEngineTrace {}

pub struct CBaseTrace {
	startpos: Vector,
	endpos: Vector,
	plane: cplane_t,
	fraction: libc::c_float,
	contents: libc::c_int,
	dispFlags: u16,
	pub allsolid: bool,
	startsolid: bool
}

pub struct trace_t {
	pub base: CBaseTrace,	// note, this is actually inheritance in C++
	fractionleftsurface: libc::c_float,
	surface: csurface_t,
	pub hitgroup: libc::c_int,
	physicsbone: libc::c_short,
	pub ent: *mut C_BaseEntity,
	hitbox: libc::c_int
}

pub struct QAngle {
	pub pitch: libc::c_float,
	pub yaw: libc::c_float,
	pub roll: libc::c_float
}
pub struct csurface_t {
	name: *const libc::c_char,
	surface_props: libc::c_short,
	flags: u16
}
pub struct cplane_t {
	normal: Vector,
	float: libc::c_float,
	type_: u8,
	signbits: u8,
	pad: [u8, ..2]
}
pub struct Vector {
	pub x: libc::c_float,
	pub y: libc::c_float,
	pub z: libc::c_float
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
pub struct Ray_t {
	data: [u8, ..128] //todo: get proper size
}
pub struct PredicateTraceFilter {
	_vmt_ptr: *const (),
	predicate: extern "C" fn (*mut IHandleEntity) -> bool
}
impl Ray_t {
	pub fn new(start: &Vector, end: &Vector) -> Ray_t {
		let mut ray: Ray_t;
		unsafe {
			ray = core::mem::uninitialized();
			ray_t_init(&mut ray, start, end);
		}
		ray
	}
}
impl C_BaseEntity {
	pub fn get_origin(&self) -> Vector {
		unsafe { c_baseentity_getorigin(self) }
	}
	pub fn get_index(&self) -> libc::c_int {
		unsafe { c_baseentity_getindex(self) }
	}
}
		
impl trace_t {
	pub unsafe fn new() -> trace_t {
		core::mem::uninitialized() // yolo
	}
}
impl Vector {
	pub fn new() -> Vector {
		Vector { x: 0f32, y: 0f32, z: 0f32 }
	}
	pub fn scale(&self, factor: f32) -> Vector {
		Vector {x: self.x * factor, y: self.y * factor, z: self.z * factor}
	}
}
impl core::ops::Add<Vector, Vector> for Vector {
	fn add(&self, rhs: &Vector) -> Vector {
		Vector {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
	}
}
impl IVEngineClient {
	pub fn client_cmd(&mut self, command: &'static str) -> Result<(), &'static str> {
		let mut buf = [0u8, ..256];
		if command.len() >= buf.len() {
			return Err("Buffer overflow!");
		}
		unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(command.repr().data), command.len()); };
		buf[command.len()] = 0;
		unsafe { ivengineclient_clientcmd(self, core::mem::transmute(buf.repr().data )) };
		
		Ok(())
	}
	pub fn time(&mut self) -> f32 {
		unsafe { ivengineclient_time(self) } 
	}
	pub fn get_local_player(&self) -> libc::c_int {
		unsafe { ivengineclient_getlocalplayer(self) }
	}
}
impl IClientEntityList {
	pub fn get_client_entity(&self, entidx: libc::c_int) -> *mut C_BaseEntity {
		unsafe {icliententitylist_getcliententity(self, entidx) }
	}
}
impl IEngineTrace {
	pub fn trace_ray(&self, ray: &Ray_t, mask: u32, filter: Option<*mut IEngineTrace>, trace: &mut trace_t) {
		
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
	fn ivengineclient_getlocalplayer(engine: &IVEngineClient) -> libc::c_int;
	
	fn icliententitylist_getcliententity(cliententitylist: *const IClientEntityList, entidx: libc::c_int) -> *mut C_BaseEntity;
	
	pub fn getptr_ibaseclientdll() -> * mut IBaseClientDLL; // MAYBE NULL
	pub fn getptr_icvar(app_sys_factory: * mut AppSysFactory) -> * mut ICvar;
	
	fn c_baseentity_getorigin(ent: *const C_BaseEntity) -> Vector;
	fn c_baseentity_getindex(ent: *const C_BaseEntity) -> libc::c_int;
	
	pub fn getptr_cinput(client: *mut IBaseClientDLL) -> *mut CInput;
	fn icvar_findvar(icvar: * mut ICvar, name: * const char) -> * mut ConVar; // MAYBE NULL;
	pub fn convar_setvalue_raw_int(cvar: * mut ConVar, value: libc::c_int);
	pub fn convar_setvalue_str(cvar: * mut ConVar, value: CString);
	pub fn convar_clearflags(cvar: * mut ConVar);
	pub fn convar_freeze(cvar: * mut ConVar);
	
	pub fn angle_vectors(angle: &QAngle, vec1: *mut Vector, vec2: *mut Vector, vec3: *mut Vector);
	
	pub fn ray_t_init(ray: &mut Ray_t, start: &Vector, end: &Vector);
	
	pub fn create_tracefilter_from_predicate(predicate: extern "C" fn(ent: *const IHandleEntity, contentsmask: i32) -> bool) -> PredicateTraceFilter;
}