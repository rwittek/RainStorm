use Cheat;
use sdk;
use libc;
use core::prelude::*;
use core;

pub struct Triggerbot;

extern "C" fn should_hit_entity(ent: *const sdk::IHandleEntity, contentsmask: i32) -> bool {
	false
}
impl Cheat for Triggerbot {
	fn new() -> Triggerbot {
		Triggerbot
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Triggerbot"
	}
	fn process_usercmd(&mut self, cmd: &mut sdk::CUserCmd) {
		// TODO: move trace_to_player into Triggerbot
		// button 1 = IN_ATTACK
		if cmd.buttons & 1 == 1 {
			cmd.buttons = !((!cmd.buttons) | 1); // zero the IN_ATTACK bit
			unsafe {
				if self.should_shoot(&cmd.viewangles) {
						cmd.buttons = cmd.buttons | 1;
				}
			}
		}
	}
}

impl Triggerbot {
	fn should_shoot(&self, viewangles: &sdk::QAngle) -> bool {
		let mut trace = sdk::trace_t::new();
		let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);

		let localplayer_entidx = ::IVENGINECLIENT_PTR.to_option().unwrap().get_local_player();
		let local_baseentity= ::ICLIENTENTITYLIST_PTR.to_option().unwrap().get_client_entity(localplayer_entidx);
		
		let me = if local_baseentity.is_not_null() {
			local_baseentity
		} else {
			log!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); libc::exit(1); 
		};

		let mut direction = sdk::Vector::new();

		sdk::angle_vectors(viewangles, &mut direction, core::ptr::mut_null(), core::ptr::mut_null());
		let eyes = me.get_origin();
		
		let eye_offsets = ((me as *mut sdk::C_BaseEntity as uint) + 0xF8) as *const [f32, ..3];
		eyes.x += (*eye_offsets)[0];
		eyes.y += (*eye_offsets)[1];
		eyes.z += (*eye_offsets)[2];
	
		direction = direction * 8192 + eyes;
		
		let ray = sdk::Ray_t::new(eyes, &direction);

		::IENGINETRACE_PTR.to_option().unwrap().trace_ray(ray, 0x200400B, &filter, &trace);
		if ( trace.base.allsolid ) {
			return false;
		}

		if ( trace.ent.is_not_null() )
		{
			let entidx = trace.ent.to_option().unwrap().index;	
			log!("Hit entity {} at hitgroup {}", entidx, trace.hitgroup);
			if (trace.hitgroup ==  1) { //&& ((*(int *)((((char *)pTrace.m_pEnt)+0x00AC)) != (*(int *)((((char *)pBaseEntity)+0x00AC)))))) {
				return true;
			}
			return false; //pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
		}

		false;
	}
}