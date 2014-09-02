use core;
use libc;
use core::ptr::RawPtr;

static VMT_MAX_SIZE_YOLO: uint = 512; // swag
type VMT = [*const (), ..VMT_MAX_SIZE_YOLO];

pub struct VMTHooker { // this should be renamed.......
	_original_vmt_ptr_ptr: *const VMT,
	original_vmt: VMT,
	patched_vmt_ptr: *mut VMT,
}

impl VMTHooker {
	pub unsafe fn new(vmt_ptr_ptr: *mut *const ()) -> VMTHooker {
		let vmt_ptr: *const VMT = core::mem::transmute(*vmt_ptr_ptr);
		// yes, we do leak this.
		// yolo.
		log!("Allocating new VMT to patch object {}...\n", vmt_ptr_ptr);
		let new_vmt = libc::malloc(core::mem::size_of::<VMT>() as u32) as *mut VMT;
		if new_vmt.is_null() {
			log!("malloc() -> NULL when trying to allocate VMT\n");
			libc::exit(1);
		} else {
			log!("VMT allocated: {}\n", new_vmt);
		}
		*new_vmt = *vmt_ptr;
		
		let hooker = VMTHooker {
			_original_vmt_ptr_ptr: core::mem::transmute(vmt_ptr_ptr),
			original_vmt: *vmt_ptr,
			patched_vmt_ptr: new_vmt
		};
		
		*vmt_ptr_ptr = (new_vmt) as *const VMT as *const ();
		
		log!("Hooray! We didn't segfault!\n");
		
		hooker
	}
	
	pub unsafe fn hook(&mut self, offset: uint, hook: *const ()) {
		log!("Patching VMT {} offset {} with {} (was: {})\n", self.patched_vmt_ptr, offset, hook, (*(self.patched_vmt_ptr))[offset]);
		(*(self.patched_vmt_ptr))[offset] = hook;
		log!("Patch OK!\n");
	}
	
	pub unsafe fn get_orig_method(&self, offset: uint) -> *const () {
		self.original_vmt[offset]
	}
}