#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub use libc::c_char;
pub use self::raw::calc_seed_from_command_number;
pub use self::raw::get_hooked_getusercmd;
pub use self::raw::{AppSysFactoryPtr};
use libc;
use core;
use core::result::{Result, Ok, Err};
use core::option::{Option, None, Some};
use core::collections::Collection;
use core::raw::Repr;
use core::mem::transmute;
use core::ptr::RawPtr;



pub use CString;

pub mod raw;
pub mod utils;

pub struct C_BaseAnimating {
	ptr: raw::C_BaseAnimatingPtr
}

pub struct C_BaseEntity {
	ptr: raw::C_BaseEntityPtr
}

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


pub fn get_icvar(appsysfactory: &AppSysFactory) -> ICvar {
	let ptr = unsafe { raw::getptr_icvar(appsysfactory.get_ptr()) };
	if ptr.is_not_null() {
		return unsafe { ICvar::from_ptr(ptr) };
	} else {
		quit!("getptr_icvar returned NULL!\n");
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
pub fn get_tracefilter(me: C_BaseEntity) -> ITraceFilter {
	unsafe { ITraceFilter::from_ptr(raw::get_tracefilter(me.get_ptr())) }
}
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
	pub ent: raw::C_BaseEntityPtr,
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
impl C_BaseEntity {
	pub unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> C_BaseEntity {
		C_BaseEntity { ptr: ptr }
	}
	pub fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	} 
	pub fn get_origin(&self) -> Vector {
		unsafe { raw::c_baseentity_getorigin(self.get_ptr()) }
	}
	pub fn worldspacecenter(&self) -> Vector {
		unsafe { raw::c_baseentity_worldspacecenter(self.get_ptr()) }
	}
	pub fn get_index(&self) -> libc::c_int {
		unsafe { raw::c_baseentity_getindex(self.get_ptr()) }
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
			let cstr_classname = raw::c_baseentity_getclassname(self.get_ptr());
			// TODO: null check?
			core::str::raw::c_str_to_static_slice(cstr_classname)
		}
	}
	pub unsafe fn mut_ptr_offset<DataType>(&mut self, offset: uint) -> *mut DataType {
		(((self.get_ptr().to_uint()) + offset) as *mut DataType)
	}
	pub unsafe fn ptr_offset<DataType>(&self, offset: uint) -> *const DataType {
		(((self.get_ptr().to_uint()) + offset) as *const DataType)
	}
}
impl core::cmp::PartialEq for C_BaseEntity {
	fn eq(&self, other: &C_BaseEntity) -> bool {
		self.ptr == other.ptr
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

impl C_BaseAnimating {
	pub unsafe fn from_ptr(ptr: raw::C_BaseAnimatingPtr) -> C_BaseAnimating {
		C_BaseAnimating { ptr: ptr }
	}
	pub fn get_ptr(&self) -> raw::C_BaseAnimatingPtr {
		self.ptr
	}
	pub fn get_hitbox_position(&self, modelinfo: IVModelInfo, hitbox: libc::c_int,
			origin: &mut Vector, angles: &QAngle) {
		unsafe { raw::c_baseanimating_gethitboxposition(self.get_ptr(), modelinfo.get_ptr(), hitbox, origin, angles) };
	}
}
impl IBaseClientDLL {
	pub unsafe fn get_ptr(&self) -> raw::IBaseClientDLLPtr {
		self.ptr
	}
	pub unsafe fn from_ptr(ptr: raw::IBaseClientDLLPtr) -> IBaseClientDLL {
		IBaseClientDLL { ptr: ptr }
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
}
impl QAngle {
	pub fn to_vector(&self) -> Vector {
		unsafe {
			let mut temp = core::mem::uninitialized();
			raw::angle_vectors(self, &mut temp, core::ptr::mut_null(), core::ptr::mut_null());
			temp
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
	pub fn get_ptr(&self) -> raw::IVEngineClientPtr {
		self.ptr
	}
	pub fn client_cmd(&mut self, command: &'static str) -> Result<(), &'static str> {
		let mut buf = [0u8, ..256];
		if command.len() >= buf.len() {
			return Err("Buffer overflow!");
		}
		unsafe { core::ptr::copy_nonoverlapping_memory(transmute::<*const u8, *mut u8>(buf.repr().data), transmute(command.repr().data), command.len()); };
		buf[command.len()] = 0;
		unsafe { raw::ivengineclient_clientcmd(self.get_ptr(), core::mem::transmute(buf.repr().data )) };
		
		Ok(())
	}
	pub fn time(&mut self) -> f32 {
		unsafe { raw::ivengineclient_time(self.get_ptr()) } 
	}
	pub fn get_local_player(&self) -> libc::c_int {
		unsafe { raw::ivengineclient_getlocalplayer(self.get_ptr()) }
	}
	pub fn get_player_name(&self, ent: C_BaseEntity, buf: &mut [u8]) -> u32 {
		unsafe {
			raw::ivengineclient_getplayername(self.get_ptr(), ent.get_ptr(), buf.repr().data as *mut u8, buf.repr().len as u32)
		}
	}
	pub fn set_viewangles(&mut self, angles: &QAngle) {
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
	pub fn get_client_entity(&self, entidx: libc::c_int) -> Option<C_BaseEntity> {
		unsafe {
			let ptr = raw::icliententitylist_getcliententity(self.get_ptr(), entidx);
			if ptr.is_not_null() {
				Some(C_BaseEntity::from_ptr(ptr))
			} else {
				None
			}
		}
	}
	pub fn get_highest_entity_index(&self) -> libc::c_int {
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
pub enum ConVarValue {
	Int(libc::c_int),
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
			Str(s) => raw::convar_setvalue_str(self.get_ptr(), s)
		}
	}
	pub unsafe fn setvalue(&mut self, val: ConVarValue) {
		match val {
			Int(v) => raw::convar_setvalue_raw_int(self.get_ptr(), v),
			Str(s) => raw::convar_setvalue_str(self.get_ptr(), s)
		}
	}
	pub unsafe fn changeandfreeze(&mut self, newval: CString) {
		raw::convar_changeandfreeze(self.get_ptr(), newval)
	}
	pub unsafe fn clearflags(&mut self) {
		raw::convar_clearflags(self.get_ptr())
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


