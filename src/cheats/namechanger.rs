use core::prelude::*;
use collections::MutableSeq;
use Cheat;
use sdk;
use GamePointers;
use rand::Rng;
use core::raw::Repr;
use sdk::Entity;

pub struct NameChanger {
	enabled: bool,
	rng: ::rand::isaac::IsaacRng,
	last_victim: i32
}

impl Cheat for NameChanger {
	fn new() -> NameChanger {
		NameChanger { enabled: false, rng: ::rand::isaac::IsaacRng::new_unseeded(), last_victim: -1 }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"NameChanger"
	}
	fn postinit(&mut self, ptrs: &GamePointers) {
		let namevar = ptrs.icvar.unwrap().find_var("name");
		match namevar {
			Some(mut name) => unsafe { name.changeandfreeze(::CString::new(::core::mem::transmute("le reddit army xD\0")).unwrap()); log!("name frozen OK :U\n") },
			None => {quit!("No name CVar? u wot m8\n")}
		}
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, _cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let localplayer_entidx = ptrs.ivengineclient.get_local_player();
		
		let me = ptrs.icliententitylist.get_client_entity(localplayer_entidx).unwrap();
		
		let icvar = ptrs.icvar.unwrap();
		let mut names: ::Vec<[u8, ..300]> = ::Vec::new();
		
		// TODO: some smart timer BS
		
		// FIXME: ugly string crappery
		for ent in sdk::utils::EntityIterator::new(ptrs.icliententitylist)
				.filter(|ptr| ptr.get_classname() == "CTFPlayer")
				.filter(|ent| unsafe { *ent.ptr_offset::<u32>(0x00AC) == *me.ptr_offset::<u32>(0x00AC) } ) {
			let mut buf = [0u8, ..300];
			let len = ptrs.ivengineclient.get_player_name(ent, buf.as_mut_slice());
			if len == 0 { return; }
			
			// Copy a zero-width space onto the end of the name.
			for (dst, src) in (buf.as_mut_slice().mut_slice_from(len as uint).mut_iter()).zip(b"\xe2\x80\x8b".iter()) {
				*dst = *src;
			}
			
			if ent.get_index() != me.get_index() && ent.get_index() != self.last_victim {
				names.push(buf);
			}
		}
		
		
		let maybe_new_name = self.rng.choose(names.as_slice());
		match maybe_new_name {
			Some(new_name) => {
				let mut name = icvar.find_var("name").expect("name cvar not found!");
				unsafe {name.setvalue_raw(
					::sdk::Str(::CString::new_raw(new_name.as_slice().repr().data as *const u8))
				)};
			},
			None => {
				// nobody else on server?
			}
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}