use core;
use log;
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
		format_args!(log, "Allocating new VMT to patch object {}...\n", vmt_ptr_ptr);
		let new_vmt = libc::malloc(core::mem::size_of::<VMT>() as u32) as *mut VMT;
		if new_vmt.is_null() {
			format_args!(log, "malloc() -> NULL when trying to allocate VMT\n");
			libc::exit(1);
		} else {
			format_args!(log, "VMT allocated: {}\n", new_vmt);
		}
		*new_vmt = *vmt_ptr;
		
		let hooker = VMTHooker {
			original_vmt_ptr_ptr: core::mem::transmute(vmt_ptr_ptr),
			original_vmt: *vmt_ptr,
			patched_vmt_ptr: new_vmt
		};
		
		*vmt_ptr_ptr = (new_vmt) as *const VMT as *const ();
		
		format_args!(log, "Hooray! We didn't segfault!\n");
		
		hooker
	}
	
	pub unsafe fn hook(&mut self, offset: uint, hook: *const ()) {
		format_args!(log, "Patching VMT {} offset {} with {}.\n", self.patched_vmt_ptr, offset, hook);
		(*(self.patched_vmt_ptr))[offset] = hook;
	}
	
	pub unsafe fn get_orig_method(&self, offset: uint) -> *const () {
		self.original_vmt[offset]
	}
}