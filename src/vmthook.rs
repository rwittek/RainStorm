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