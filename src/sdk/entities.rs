use super::raw;
use super::{Vector, IVModelInfo, TFClass};
use core;
use core::num::FromPrimitive;

pub trait Entity: core::kinds::Copy {
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> Self;
	fn get_ptr(&self) -> raw::C_BaseEntityPtr;
	
	fn get_origin(&self) -> Vector {
		unsafe { raw::c_baseentity_getorigin(self.get_ptr()) }
	}
	fn worldspacecenter(&self) -> Vector {
		unsafe { raw::c_baseentity_worldspacecenter(self.get_ptr()) }
	}
	fn get_velocity(&self) -> Vector {
		unsafe { raw::c_baseentity_getvelocity(self.get_ptr()) }
	}
	fn interpolate(&self, time: f32) {
		unsafe { raw::c_baseentity_interpolate(self.get_ptr(), time) }
	}
	fn get_index(&self) -> i32 {
		unsafe { raw::c_baseentity_getindex(self.get_ptr()) }
	}
	
	fn get_classname<'a>(&'a self) -> &'a str {
		unsafe {
			let cstr_classname = raw::c_baseentity_getclassname(self.get_ptr());
			// TODO: null check?
			core::str::raw::c_str_to_static_slice(cstr_classname)
		}
	}
	fn mut_ptr_offset<DataType>(&mut self, offset: uint) -> *mut DataType {
		(((self.get_ptr().to_uint()) + offset) as *mut DataType)
	}
	fn ptr_offset<DataType>(&self, offset: uint) -> *const DataType {
		(((self.get_ptr().to_uint()) + offset) as *const DataType)
	}
}

/*impl<EntityType: BaseEntity> core::cmp::PartialEq for EntityType {
	fn eq(&self, other: &EntityType) -> bool {
		self.get_index() == other.get_index()
	}
}*/

pub trait Animating: Entity {
	fn get_hitbox_position(&mut self, modelinfo: IVModelInfo, hitbox: i32) -> Vector {
		unsafe { 
	//		log!("animating: {}, {}\n", *self.ptr_offset::<f32>(0x07F8), *self.ptr_offset::<f32>(0x07F8 + 0x14));
	//		*self.mut_ptr_offset::<f32>(0x07F8) += 1.0/66.0;
			
			let mut origin = core::mem::uninitialized();
			raw::c_baseanimating_gethitboxposition(self.get_ptr(), modelinfo.get_ptr(), hitbox, &mut origin);
			
		//	*self.mut_ptr_offset::<f32>(0x07F8) -= 1.0 / 66.0; // reverse tick update
			
			origin
		}
	}
	fn get_bone_position(&self, modelinfo: IVModelInfo, bone: i32) -> Vector {
		unsafe {
			let mut origin = core::mem::uninitialized();
			raw::c_baseanimating_getboneposition(self.get_ptr(), modelinfo.get_ptr(), bone, &mut origin);
			origin
		}
	}
	fn get_num_bones(&self, modelinfo: IVModelInfo) -> i32 {
		unsafe {
			raw::c_baseanimating_getnumbones(self.get_ptr(), modelinfo.get_ptr())
		}
	}
	fn get_num_hitboxes(&self, modelinfo: IVModelInfo) -> i32 {
		unsafe {
			raw::c_baseanimating_getnumhitboxes(self.get_ptr(), modelinfo.get_ptr())
		}
	}
}

pub trait OnTeam: Entity {
	fn get_team(&self) -> u32;
}

pub trait CombatWeapon: Animating {
	fn is_melee(&self) -> bool;
}
// FIXME: this is weird
pub struct BaseCombatWeapon {
	ptr: raw::C_BaseEntityPtr
}
impl Entity for BaseCombatWeapon {
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	}
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> BaseCombatWeapon {
		BaseCombatWeapon {ptr: ptr}
	}
}
impl Animating for BaseCombatWeapon {}
impl CombatWeapon for BaseCombatWeapon {
	fn is_melee(&self) -> bool {
		false // FIXME
	}
}

pub struct TFPlayer {
	ptr: raw::C_BaseEntityPtr
}

impl TFPlayer {
	pub fn get_life_state(&self) -> i8 {
		unsafe { *(self.ptr_offset::<i8>(0x00A1)) }
	}
	pub fn get_class(&self) -> TFClass {
		let classnum = unsafe {*(self.ptr_offset::<u32>(0x1528))};
		FromPrimitive::from_u32(classnum).expect("Invalid class number?")
	}
	pub fn get_health(&self) -> i32 {
		unsafe { *self.ptr_offset(0x00A4) }
	}
}
impl Entity for TFPlayer {
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	}
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> TFPlayer {
		TFPlayer {ptr: ptr}
	}
}
impl Animating for TFPlayer {}
impl<T: Entity> OnTeam for T {
	fn get_team(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x00AC))}
	}
}

pub trait Object: Animating + OnTeam {}

pub struct BaseObject {
	ptr: raw::C_BaseEntityPtr
}
impl Entity for BaseObject {
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	}
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> BaseObject {
		BaseObject {ptr: ptr}
	}
}
impl Animating for BaseObject {}
/*impl OnTeam for BaseObject {
	fn get_team(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x00AC))}
	}
}*/
impl Object for BaseObject {}