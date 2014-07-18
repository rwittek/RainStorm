use core;
use libc;
use core::ptr::RawPtr;

static VMT_MAX_SIZE_YOLO: u32 = 512; // swag
type VMT = [*const (), ..VMT_MAX_SIZE_YOLO];

pub struct VMTHooker { // this should be renamed.......
	original_vmt: VMT,
	patched_vmt: VMT,
}

impl VMTHooker {
	pub unsafe fn new(vmt_ptr_ptr: *mut *const ()) -> VMTHooker {
		let vmt_ptr: *const VMT = core::mem::transmute(*vmt_ptr_ptr);
		log!("Patching VMT {} from object {}...\n", vmt_ptr, vmt_ptr_ptr);

		let hooker = VMTHooker {
			original_vmt: *vmt_ptr,
			patched_vmt: *vmt_ptr
		};
		
		*vmt_ptr_ptr = &hooker.patched_vmt as *const VMT as *const ();
		
		log!("Hooray! We didn't segfault!\n");
		
		hooker
	}
	
	pub unsafe fn hook(&mut self, offset: uint, hook: *const ()) {
		log!("Patching VMT offset {} with {} (was: {})\n",  offset, hook, (self.patched_vmt[offset]));
		self.patched_vmt[offset] = hook;
	}
	
	pub unsafe fn get_orig_method(&self, offset: uint) -> *const () {
		self.original_vmt[offset]
	}
}