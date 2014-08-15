#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub use libc::c_char;
pub use self::raw::calc_seed_from_command_number;
pub use self::raw::get_hooked_getusercmd;
pub use self::raw::{AppSysFactoryPtr};

pub use self::entities::{Entity, Animating, BaseCombatWeapon, CombatWeapon, TFPlayer, OnTeam, Object, BaseObject};

use libc;
use core;
use core::result::{Result, Ok, Err};
use core::option::{Option, None, Some};
use core::collections::Collection;
use core::raw::Repr;
use core::mem::transmute;
use core::ptr::RawPtr;



pub use CString;

pub mod entities;

pub mod raw;
pub mod utils;

pub struct ITraceFilter {
	ptr: raw::ITraceFilterPtr
}

pub struct IVModelInfo {
	ptr: raw::IVModelInfoPtr
}
pub struct AppSysFactory {
	ptr: raw::AppSysFactoryPtr
}
pub struct CInput {
	ptr: raw::CInputPtr
}
pub struct IBaseClientDLL {
	ptr: raw::IBaseClientDLLPtr
}
pub struct ICvar {
	ptr: raw::ICvarPtr
}
pub struct IVEngineClient {
	ptr: raw::IVEngineClientPtr
}
pub struct IClientEntityList {
	ptr: raw::IClientEntityListPtr
}
pub struct IEngineTrace {
	ptr: raw::IEngineTracePtr
}
pub struct ConVar {
	ptr: raw::ConVarPtr
}

pub struct IUniformRandomStream {
	ptr: raw::IUniformRandomStreamPtr
}

pub fn get_icvar(appsysfactory: &AppSysFactory) -> ICvar {
	let ptr = unsafe { raw::getptr_icvar(appsysfactory.get_ptr()) };
	if ptr.is_not_null() {
		return unsafe { ICvar::from_ptr(ptr) };
	} else {
		quit!("getptr_icvar returned NULL!\n");
	}
}
pub fn get_iuniformrandomstream() -> IUniformRandomStream {
	let ptr = unsafe { raw::getptr_iuniformrandomstream() };
	if ptr.is_not_null() {
		return unsafe { IUniformRandomStream::from_ptr(ptr) };
	} else {
		quit!("getptr_iuniformrandomstream returned NULL!\n");
	}
}
pub fn get_ivengineclient() -> IVEngineClient {
	let ptr = unsafe { raw::getptr_ivengineclient() };
	if ptr.is_not_null() {
		return unsafe { IVEngineClient::from_ptr(ptr) };
	} else {
		quit!("getptr_ivengineclient returned NULL!\n");
	}
}
pub fn get_icliententitylist() -> IClientEntityList {
	let ptr = unsafe { raw::getptr_icliententitylist() };
	if ptr.is_not_null() {
		return unsafe { IClientEntityList::from_ptr(ptr) };
	} else {
		quit!("getptr_icliententitylist returned NULL!\n");
	}
}
pub fn get_ibaseclientdll() -> IBaseClientDLL {
	let ptr = unsafe { raw::getptr_ibaseclientdll() };
	if ptr.is_not_null() {
		return unsafe { IBaseClientDLL::from_ptr(ptr) };
	} else {
		quit!("getptr_ibaseclientdll returned NULL!\n");
	}
}
pub fn get_ienginetrace() -> IEngineTrace {
	let ptr = unsafe { raw::getptr_ienginetrace() };
	if ptr.is_not_null() {
		return unsafe { IEngineTrace::from_ptr(ptr) };
	} else {
		quit!("getptr_ienginetrace returned NULL!\n");
	}
}
pub fn get_ivmodelinfo() -> IVModelInfo {
	let ptr = unsafe { raw::getptr_ivmodelinfo() };
	if ptr.is_not_null()	{
		return unsafe { IVModelInfo::from_ptr(ptr) };
	} else {
		quit!("getptr_ivmodelinfo returned NULL!\n");
	}
}
pub fn get_tracefilter<EntType: Entity>(me: EntType) -> ITraceFilter {
	unsafe { ITraceFilter::from_ptr(raw::get_tracefilter(me.get_ptr())) }
}
#[deriving(FromPrimitive, Show)]
pub enum TFClass {
	Scout = 1,
	Sniper,
	Soldier,
	Demoman,
	Medic,
	Heavy,
	Pyro,
	Spy,
	Engineer, // lol wtf
	// Bread
}
pub static IN_ATTACK: i32 = (1 << 0);
pub static IN_JUMP: i32 = (1 << 1);
pub static IN_DUCK: i32 = (1 << 2);
pub static IN_FORWARD: i32 = (1 << 3);
pub static IN_BACK: i32 = (1 << 4);
pub static IN_USE: i32 = (1 << 5);
pub static IN_CANCEL: i32 = (1 << 6);
pub static IN_LEFT: i32 = (1 << 7);
pub static IN_RIGHT: i32 = (1 << 8);
pub static IN_MOVELEFT: i32 = (1 << 9);
pub static IN_MOVERIGHT: i32 = (1 << 10);
pub static IN_ATTACK2: i32 = (1 << 11);
pub static IN_RUN: i32 = (1 << 12);
pub static IN_RELOAD: i32 = (1 << 13);
pub static IN_ALT1: i32 = (1 << 14);
pub static IN_ALT2: i32 = (1 << 15);
pub static IN_SCORE: i32 = (1 << 16); // Used by client.dll for when scoreboard is held down
pub static IN_SPEED: i32 = (1 << 17); // Player is holding the speed key
pub static IN_WALK: i32 = (1 << 18); // Player holding walk key
pub static IN_ZOOM: i32 = (1 << 19); // Zoom key for HUD zoom
pub static IN_WEAPON1: i32 = (1 << 20);// weapon defines these bits
pub static IN_WEAPON2: i32 = (1 << 21);// weapon defines these bits
pub static IN_BULLRUSH: i32 = (1 << 22);

pub struct CBaseHandle {
	index: libc::c_long
}

pub struct CBaseTrace {
	startpos: Vector,
	endpos: Vector,
	plane: cplane_t,
	fraction: libc::c_float,
	contents: i32,
	dispFlags: u16,
	pub allsolid: bool,
	startsolid: bool
}

pub struct trace_t {
	pub base: CBaseTrace,	// note, this is actually inheritance in C++
	fractionleftsurface: libc::c_float,
	surface: csurface_t,
	pub hitgroup: i32,
	pub physicsbone: libc::c_short,
	pub ent: raw::C_BaseEntityPtr,
	pub hitbox: i32
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

impl AppSysFactory {
	pub fn get_ptr(&self) -> raw::AppSysFactoryPtr {
		self.ptr
	}
	pub unsafe fn from_ptr(ptr: raw::AppSysFactoryPtr) -> AppSysFactory {
		AppSysFactory { ptr: ptr }
	}
}
impl ITraceFilter {
	pub unsafe fn from_ptr(ptr: raw::ITraceFilterPtr) -> ITraceFilter {
		ITraceFilter { ptr: ptr }
	}
		pub fn get_ptr(&self) -> raw::ITraceFilterPtr {
		self.ptr
	}
}
impl Ray_t {
	pub fn new(start: &Vector, end: &Vector) -> Ray_t {
		let mut ray: Ray_t;
		unsafe {
			ray = core::mem::uninitialized();
			raw::ray_t_init(&mut ray, start, end);
		}
		ray
	}
}
impl Entity for raw::C_BaseEntityPtr {
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> raw::C_BaseEntityPtr {
		ptr
	}
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		*self
	} 
}


impl IVModelInfo {
	pub unsafe fn get_ptr(&self) -> raw::IVModelInfoPtr {
		self.ptr
	}
	pub unsafe fn from_ptr(ptr: raw::IVModelInfoPtr) -> IVModelInfo {
		IVModelInfo { ptr: ptr }
	}
}

impl IBaseClientDLL {
	pub fn get_ptr(&self) -> raw::IBaseClientDLLPtr {
		self.ptr
	}
	pub unsafe fn from_ptr(ptr: raw::IBaseClientDLLPtr) -> IBaseClientDLL {
		IBaseClientDLL { ptr: ptr }
	}
	pub fn set_crosshair_angles(self, angles: &QAngle) {
		unsafe {
			raw::ibaseclientdll_setcrosshairangles(self.get_ptr(), angles)
		}
	}
}
impl IUniformRandomStream {
	pub fn get_ptr(&self) -> raw::IUniformRandomStreamPtr {
		self.ptr
	}
	pub unsafe fn from_ptr(ptr: raw::IUniformRandomStreamPtr) -> IUniformRandomStream {
		IUniformRandomStream { ptr: ptr }
	}
	pub fn set_seed(&self, seed: i32) {
		unsafe {
			raw::iuniformrandomstream_set_seed(self.get_ptr(), seed);
		}
	}
	pub fn random_int(&self, minval: i32, maxval: i32) -> i32 {
		unsafe {
			raw::iuniformrandomstream_random_int(self.get_ptr(), minval, maxval)
		}
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
		unsafe { raw::vector_length(self) }
	}
	pub fn to_angle(&self) -> QAngle {
		unsafe {
			let mut temp = core::mem::uninitialized();
			raw::vector_angles(self, &mut temp);
			temp
		}
	}
	pub fn norm(&self) -> Vector {
		let len = self.length();
		Vector {
			x: self.x / len,
			y: self.y / len,
			z: self.z / len
		}
	}
			
	pub fn dotproduct(&self, other: &Vector) -> f32 {
		self.x * other.x
		+ self.y * other.y
		+ self.z * other.z
	}
			
}
impl QAngle {
	pub fn to_vector(&self) -> Vector {
		unsafe {
			let mut temp = core::mem::uninitialized();
			raw::angle_vectors(self, &mut temp, core::ptr::mut_null(), core::ptr::mut_null());
			temp
		}
	}
	pub fn to_vectors(&self) -> (Vector, Vector, Vector) {
		unsafe {
			let (mut fwd, mut right, mut up) = (core::mem::uninitialized(), core::mem::uninitialized(), core::mem::uninitialized());
			raw::angle_vectors(self, &mut fwd, &mut right, &mut up);
			(fwd, right, up)
		}
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
	/// ptr MUST be valid!
	pub unsafe fn from_ptr(ptr: raw::IVEngineClientPtr) -> IVEngineClient {
		IVEngineClient { ptr: ptr }
	}
	pub fn get_ptr(self) -> raw::IVEngineClientPtr {
		self.ptr
	}
	pub fn client_cmd(self, command: &'static str) -> Result<(), &'static str> {
		let mut buf = [0u8, ..256];
		if command.len() >= buf.len() {
			return Err("Buffer overflow!");
		}
		unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(command.repr().data), command.len()); };
		buf[command.len()] = 0;
		unsafe { raw::ivengineclient_clientcmd(self.get_ptr(), core::mem::transmute(buf.repr().data )) };
		
		Ok(())
	}
	pub fn time(self) -> f32 {
		unsafe { raw::ivengineclient_time(self.get_ptr()) } 
	}
	pub fn get_local_player(self) -> i32 {
		unsafe { raw::ivengineclient_getlocalplayer(self.get_ptr()) }
	}
	pub fn get_player_name<EntType: Entity>(self, ent: EntType, buf: &mut [u8]) -> u32 {
		unsafe {
			raw::ivengineclient_getplayername(self.get_ptr(), ent.get_ptr(), buf.repr().data as *mut u8, buf.repr().len as u32)
		}
	}
	pub fn set_viewangles(self, angles: &QAngle) {
		unsafe {
			raw::ivengineclient_setviewangles(self.get_ptr(), angles)
		}
	}
}
impl IClientEntityList {
	pub unsafe fn from_ptr(ptr: raw::IClientEntityListPtr) -> IClientEntityList {
		IClientEntityList { ptr: ptr }
	}
	pub fn get_ptr(&self) -> raw::IClientEntityListPtr {
		self.ptr
	}
	pub fn get_client_entity(&self, entidx: i32) -> Option<raw::C_BaseEntityPtr> {
		unsafe {
			let ptr = raw::icliententitylist_getcliententity(self.get_ptr(), entidx);
			if ptr.is_not_null() {
				Some(ptr)
			} else {
				None
			}
		}
	}

	pub fn get_client_entity_from_handle(&self, handle: CBaseHandle) -> Option<raw::C_BaseEntityPtr> {
		unsafe {
			let ptr = raw::icliententitylist_getcliententityfromhandle(self.get_ptr(), handle);
			if ptr.is_not_null() {
				Some(ptr)
			} else {
				None
			}
		}
	}
	pub fn get_highest_entity_index(&self) -> i32 {
		unsafe { raw::icliententitylist_get_highest_entity_index(self.get_ptr()) }
	}
}
impl ICvar {
	/// ptr MUST be valid!
	pub unsafe fn from_ptr(ptr: raw::ICvarPtr) -> ICvar {
		ICvar { ptr: ptr }
	}
}

impl IEngineTrace {
	pub unsafe fn from_ptr(ptr: raw::IEngineTracePtr) -> IEngineTrace {
		IEngineTrace { ptr: ptr }
	}
	pub unsafe fn get_ptr(&self) -> raw::IEngineTracePtr {
		self.ptr
	}
	
	pub fn trace_ray(&self, ray: &Ray_t, mask: u32, filter: Option<ITraceFilter>, trace: &mut trace_t) {
		let filter_ptr = match filter {
			Some(filter) => filter.get_ptr(),
			None => raw::ITraceFilterPtr::null()
		};
		unsafe { raw::ienginetrace_traceray(self.get_ptr(), ray, mask, filter_ptr, trace) };
	}
}

// Fixme: should be a trait
pub enum ConVarValue {
	Int(i32),
	Float(f32),
	Str(CString)
}

impl ConVar {
	pub unsafe fn from_ptr(ptr: raw::ConVarPtr) -> ConVar {
		ConVar { ptr: ptr }
	}
	pub fn get_ptr(&self) -> raw::ConVarPtr {
		self.ptr
	}
	pub unsafe fn setvalue_raw(&mut self, val: ConVarValue) {
		match val {
			Int(v) => raw::convar_setvalue_raw_int(self.get_ptr(), v),
			Float(f) => raw::convar_setvalue_float(self.get_ptr(), f),
			Str(s) => raw::convar_setvalue_str(self.get_ptr(), s),
		}
	}
	pub fn setvalue(&mut self, val: ConVarValue) {
		unsafe {
			match val {
				Int(v) => raw::convar_setvalue_raw_int(self.get_ptr(), v),
				Float(f) => raw::convar_setvalue_float(self.get_ptr(), f),
				Str(s) => raw::convar_setvalue_str(self.get_ptr(), s)
			}
		}
	}
	pub fn getvalue_float(&self) -> ConVarValue {
		unsafe {
			Float(raw::convar_getvalue_float(self.get_ptr()))
		}
	}
	pub unsafe fn changeandfreeze(&mut self, newval: CString) {
		raw::convar_changeandfreeze(self.get_ptr(), newval)
	}
	pub fn clearflags(&mut self) {
		unsafe { raw::concommand_clearflags(self.get_ptr()) }
	}
}

impl ICvar {
	pub fn find_var(&self, name: &str) -> Option<ConVar> {
		let mut buf = [0u8, ..256];
		if name.len() >= buf.len() {
			return None
		} else {
			unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(name.repr().data), name.len()); }
			buf[name.len()] = 0;
			let raw_convar = unsafe { raw::icvar_findvar(self.get_ptr(), transmute(buf.repr().data)) };
			match raw_convar.is_null() {
				true => None,
				false => unsafe { Some(ConVar::from_ptr(raw_convar)) }
			}
		}
	}
	pub fn get_ptr(&self) -> raw::ICvarPtr {
		self.ptr
	}
}


