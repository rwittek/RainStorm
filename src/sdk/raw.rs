use libc;
use core::prelude::*;
use super::{trace_t, Ray_t, Vector, QAngle};
use CString;
use core;

// the lifetime of these is "hopefully long enough"
macro_rules! ptr_wrapper(
	($name:ident) => (
		#[allow(raw_pointer_deriving)]
		#[deriving(Show)]
		pub struct $name (*mut ());
		impl core::cmp::PartialEq for $name {
			fn eq(&self, other: &$name) -> bool {
				let &$name(ref selfptr) = self;
				let &$name(ref otherptr) = other;
				selfptr == otherptr
			}
		}
		impl $name {
			pub fn null() -> $name {
				$name(core::ptr::RawPtr::null())
			}
			pub fn is_null(&self) -> bool {
				self == &$name::null()
			}
			pub fn is_not_null(&self) -> bool {
				self != &$name::null()
			}
			pub fn to_uint(&self) -> uint {
				let &$name(ref ptr) = self;
				(*ptr) as uint
			}
			pub unsafe fn from_uint(val: uint) -> $name {
				$name(val as *mut ())
			}
		}
	)
)

ptr_wrapper!(IVEngineClientPtr)
ptr_wrapper!(IBaseClientDLLPtr)
ptr_wrapper!(ConVarPtr)
ptr_wrapper!(ICvarPtr)
ptr_wrapper!(AppSysFactoryPtr)
ptr_wrapper!(PhysicsFactoryPtr)
ptr_wrapper!(GlobalsPtr)
ptr_wrapper!(CInputPtr)
ptr_wrapper!(C_BaseEntityPtr)
ptr_wrapper!(C_BaseAnimatingPtr)
ptr_wrapper!(IHandleEntityPtr)
ptr_wrapper!(IClientEntityListPtr)
ptr_wrapper!(IEngineTracePtr)
ptr_wrapper!(IVModelInfoPtr)
ptr_wrapper!(INetChannelPtr)
ptr_wrapper!(INetMessagePtr)
ptr_wrapper!(ITraceFilterPtr)

extern "C" {
	pub fn getptr_ivengineclient() -> IVEngineClientPtr; // MAYBE NULL
	pub fn ivengineclient_clientcmd(engine: IVEngineClientPtr, cmd_string: * const libc::c_char);
	pub fn ivengineclient_time(engine: IVEngineClientPtr) -> libc::c_float;
	pub fn ivengineclient_getlocalplayer(engine: IVEngineClientPtr) -> libc::c_int;
	pub fn ivengineclient_getplayername(engine: IVEngineClientPtr, ent: C_BaseEntityPtr, buf: *mut u8, bufsize: libc::size_t) -> libc::size_t;
	pub fn ivengineclient_setviewangles(engine: IVEngineClientPtr, angles: &QAngle);

	pub fn getptr_ienginetrace() -> IEngineTracePtr; // MAYBE NULL
	pub fn ienginetrace_traceray(enginetrace: IEngineTracePtr, ray: &Ray_t, mask: u32, filter: ITraceFilterPtr, trace: &mut trace_t);
	
	pub fn getptr_icliententitylist() -> IClientEntityListPtr; // MAYBE NULL
	pub fn icliententitylist_getcliententity(cliententitylist: IClientEntityListPtr, entidx: libc::c_int) -> C_BaseEntityPtr;
	pub fn icliententitylist_get_highest_entity_index(entlist: IClientEntityListPtr) -> libc::c_int;
	
	pub fn getptr_ibaseclientdll() -> IBaseClientDLLPtr; // MAYBE NULL
	pub fn getptr_icvar(app_sys_factory: AppSysFactoryPtr) -> ICvarPtr;
	
	pub fn trace_t_gethitgroup(trace: *const trace_t) -> libc::c_int;
	pub fn c_baseentity_getorigin(ent: C_BaseEntityPtr) -> Vector;
	pub fn c_baseentity_worldspacecenter(ent: C_BaseEntityPtr) -> Vector;
	pub fn c_baseentity_getindex(ent: C_BaseEntityPtr) -> libc::c_int;
	pub fn c_baseentity_getclassname(ent: C_BaseEntityPtr) -> *const libc::c_char;
	
	pub fn getptr_ivmodelinfo() -> IVModelInfoPtr;
	
	pub fn c_baseanimating_gethitboxposition(ent: C_BaseAnimatingPtr, modelinfo: IVModelInfoPtr, hitbox: libc::c_int,
		origin: &mut Vector);
	pub fn c_baseanimating_getboneposition(ent: C_BaseAnimatingPtr, modelinfo: IVModelInfoPtr, bone: libc::c_int,
		origin: &mut Vector);
	pub fn c_baseanimating_getnumbones(ent: C_BaseAnimatingPtr, modelinfo: IVModelInfoPtr) -> libc::c_int;
	pub fn c_baseanimating_getnumhitboxes(ent: C_BaseAnimatingPtr, modelinfo: IVModelInfoPtr) -> libc::c_int;
	
	pub fn getptr_cinput(client: IBaseClientDLLPtr) -> CInputPtr;
	pub fn icvar_findvar(icvar: ICvarPtr, name: * const char) -> ConVarPtr; // MAYBE NULL;
	pub fn convar_setvalue_raw_int(cvar: ConVarPtr, value: libc::c_int);
	pub fn convar_setvalue_str(cvar: ConVarPtr, value: CString);
	pub fn convar_clearflags(cvar: ConVarPtr);
	pub fn convar_changeandfreeze(cvar: ConVarPtr, newval: CString);
	
	pub fn angle_vectors(angle: &QAngle, vec1: *mut Vector, vec2: *mut Vector, vec3: *mut Vector);
	pub fn vector_angles(vector: &Vector, angles: &mut QAngle);
	pub fn vector_length(vector: &Vector) -> libc::c_float;
	pub fn ray_t_init(ray: &mut Ray_t, start: &Vector, end: &Vector);
	
	//pub fn create_tracefilter_from_predicate(predicate: extern "C" pub fn(ent: *const IHandleEntity, contentsmask: i32) -> bool) -> PredicateTraceFilter;
	
	pub fn get_current_inetchannel(engine: IVEngineClientPtr) -> INetChannelPtr;
	pub fn get_current_latency(engine: IVEngineClientPtr) -> libc::c_float;
	pub fn get_netchannel_sendnetmsg_trampoline() -> *const ();
	pub fn get_hooked_getusercmd() -> *const ();
	pub fn ismousedown() -> bool;
	
	pub fn calc_seed_from_command_number(cmdnum: libc::c_int) -> libc::c_int;
	pub fn get_tracefilter(me: C_BaseEntityPtr) -> ITraceFilterPtr;
}