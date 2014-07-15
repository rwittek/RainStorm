use Cheat;
use CheatManager;
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
	fn process_usercmd(&mut self, mgr: &CheatManager, cmd: &mut sdk::CUserCmd) {
		let mut ivengineclient = unsafe { mgr.ivengineclient_ptr.to_option().unwrap() };
		let mut icliententitylist = unsafe { mgr.icliententitylist_ptr.to_option().unwrap() };
		let mut ienginetrace = unsafe { mgr.ienginetrace_ptr.to_option().unwrap() };
		// button 1 = IN_ATTACK
		if cmd.buttons & 1 == 1 {
			cmd.buttons = !((!cmd.buttons) | 1); // zero the IN_ATTACK bit
			unsafe {
				if self.should_shoot(ivengineclient, icliententitylist, ienginetrace, &cmd.viewangles) {
						cmd.buttons = cmd.buttons | 1;
				}
			}
		}
	}
}

impl Triggerbot {
	fn should_shoot(&self, ivengineclient: &mut sdk::IVEngineClient, icliententitylist: &mut sdk::IClientEntityList,
			ienginetrace: &mut sdk::IEngineTrace, viewangles: &sdk::QAngle) -> bool {
		let mut trace = unsafe { sdk::trace_t::new() };
		//let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);

		let localplayer_entidx = ivengineclient.get_local_player();
		let local_baseentity = icliententitylist.get_client_entity(localplayer_entidx);
		
		let me: &mut sdk::C_BaseEntity = if local_baseentity.is_not_null() {
			unsafe { core::mem::transmute(local_baseentity) }
		} else {
			log!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); unsafe { libc::exit(1) }; 
		};

		let mut direction = sdk::Vector::new();

		unsafe {
			sdk::angle_vectors(viewangles, &mut direction, core::ptr::mut_null(), core::ptr::mut_null());
		}
		let mut eyes = me.get_origin();
		
		unsafe {
			let eye_offsets = ((me as *mut sdk::C_BaseEntity as uint) + 0xF8) as *const [f32, ..3];
			eyes.x += (*eye_offsets)[0];
			eyes.y += (*eye_offsets)[1];
			eyes.z += (*eye_offsets)[2];
		}
		direction = direction.scale( 8192.0f32 ) + eyes;
		
		let ray = sdk::Ray_t::new(&eyes, &direction);

		ienginetrace.trace_ray(&ray, 0x46004001, None, &mut trace);
		if ( trace.base.allsolid ) {
			return false;
		}

		if ( trace.ent.is_not_null() )
		{
			let entidx = unsafe { trace.ent.to_option().unwrap().get_index() };	
			//log!("Hit entity {} at hitgroup {}\n", entidx, unsafe { sdk::trace_t_gethitgroup(&trace)});
			if (trace.hitgroup ==  1) && 
					unsafe {
						((*(((trace.ent as uint)+0x00AC) as *const i32)) != (*(((me as *mut sdk::C_BaseEntity as uint)+0x00AC) as *const i32)))
					}
			{
				return true;
			}
			return false; //pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
		}

		false
	}
}