#![allow(non_camel_case_types)]
#![allow(dead_code)]
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

mod raw;

pub static IN_ATTACK: libc::c_int = (1 << 0);
pub static IN_JUMP: libc::c_int = (1 << 1);
pub static IN_DUCK: libc::c_int = (1 << 2);
pub static IN_FORWARD: libc::c_int = (1 << 3);
pub static IN_BACK: libc::c_int = (1 << 4);
pub static IN_USE: libc::c_int = (1 << 5);
pub static IN_CANCEL: libc::c_int = (1 << 6);
pub static IN_LEFT: libc::c_int = (1 << 7);
pub static IN_RIGHT: libc::c_int = (1 << 8);
pub static IN_MOVELEFT: libc::c_int = (1 << 9);
pub static IN_MOVERIGHT: libc::c_int = (1 << 10);
pub static IN_ATTACK2: libc::c_int = (1 << 11);
pub static IN_RUN: libc::c_int = (1 << 12);
pub static IN_RELOAD: libc::c_int = (1 << 13);
pub static IN_ALT1: libc::c_int = (1 << 14);
pub static IN_ALT2: libc::c_int = (1 << 15);
pub static IN_SCORE: libc::c_int = (1 << 16); // Used by client.dll for when scoreboard is held down
pub static IN_SPEED: libc::c_int = (1 << 17); // Player is holding the speed key
pub static IN_WALK: libc::c_int = (1 << 18); // Player holding walk key
pub static IN_ZOOM: libc::c_int = (1 << 19); // Zoom key for HUD zoom
pub static IN_WEAPON1: libc::c_int = (1 << 20);// weapon defines these bits
pub static IN_WEAPON2: libc::c_int = (1 << 21);// weapon defines these bits
pub static IN_BULLRUSH: libc::c_int = (1 << 22);

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
	pub physicsbone: libc::c_short,
	pub ent: *mut C_BaseEntity,
	pub hitbox: libc::c_int
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
#[deriving(Show)]
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

	pub random_seed: i32,

	pub mousedx: u16,
	pub mousedy: u16,

	pub hasbeenpredicted: bool
}
pub struct Ray_t {
	data: [u8, ..128] //todo: get proper size
}
pub struct PredicateTraceFilter {
	_vmt_ptr: *const (), // do NOT mirror in C++
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
	pub fn worldspacecenter(&self) -> Vector {
		unsafe { c_baseentity_worldspacecenter(self) }
	}
	pub fn get_index(&self) -> libc::c_int {
		unsafe { c_baseentity_getindex(self) }
	}
	pub fn get_life_state(&self) -> i8 {
		unsafe { *(self.ptr_offset::<i8>(0x00A1)) }
	}
	pub fn get_team(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x00AC))}
	}
	pub fn get_class(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x1524))}
	}
	pub fn get_classname<'a>(&'a self) -> &'a str {
		unsafe {
			let cstr_classname = c_baseentity_getclassname(self);
			// TODO: null check?
			core::str::raw::c_str_to_static_slice(cstr_classname)
		}
	}
	pub unsafe fn mut_ptr_offset<DataType>(&mut self, offset: uint) -> *mut DataType {
		(((self as *mut C_BaseEntity as uint) + offset) as *mut DataType)
	}
	pub unsafe fn ptr_offset<DataType>(&self, offset: uint) -> *const DataType {
		(((self as *const C_BaseEntity as uint) + offset) as *const DataType)
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
	pub fn length(&self) -> f32 {
		unsafe { vector_length(self) }
	}
}
impl core::ops::Add<Vector, Vector> for Vector {
	fn add(&self, rhs: &Vector) -> Vector {
		Vector {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
	}
}
impl core::ops::Sub<Vector, Vector> for Vector {
	fn sub(&self, rhs: &Vector) -> Vector {
		Vector {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
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
	pub fn get_player_name(&self, ent: &C_BaseEntity, buf: &mut [u8]) -> u32 {
		unsafe {
			ivengineclient_getplayername(self, ent, buf.repr().data as *mut u8, buf.repr().len as u32)
		}
	}
	pub fn set_viewangles(&mut self, angles: &QAngle) {
		unsafe {
			ivengineclient_setviewangles(self, angles)
		}
	}
}
impl IClientEntityList {
	pub fn get_client_entity(&self, entidx: libc::c_int) -> *mut C_BaseEntity {
		unsafe {icliententitylist_getcliententity(self, entidx) }
	}
	pub fn get_highest_entity_index(&self) -> libc::c_int {
		unsafe { icliententitylist_get_highest_entity_index(self) }
	}
}
impl IEngineTrace {
	pub fn trace_ray(&self, ray: &Ray_t, mask: u32, filter: Option<&mut ITraceFilter>, trace: &mut trace_t) {
		let filter_ptr = match filter {
			Some(ptr) => ptr as *mut ITraceFilter,
			None => core::ptr::mut_null()
		};
		unsafe { ienginetrace_traceray(self, ray, mask, filter_ptr, trace) };
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
	pub unsafe fn changeandfreeze(&mut self, newval: CString) {
		convar_changeandfreeze(self as *mut ConVar, newval)
	}
	pub unsafe fn clearflags(&mut self) {
		convar_clearflags(self as *mut ConVar)
	}
}

impl ICvar {
	pub fn find_var(&self, name: &str) -> Option<*mut ConVar> {
		let mut buf = [0u8, ..256];
		if name.len() >= buf.len() {
			return None
		} else {
			unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(name.repr().data), name.len()); }
			buf[name.len()] = 0;
			let raw_convar = unsafe { icvar_findvar(self as *const ICvar, transmute(buf.repr().data)) };
			match raw_convar.is_null() {
				true => None,
				false => Some(raw_convar)
			}
		}
	}
}


