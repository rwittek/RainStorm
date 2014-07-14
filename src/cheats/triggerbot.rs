use Cheat;
use sdk;

pub struct Triggerbot;

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
				if self.should_shoot(cmd.viewangles) {
						cmd.buttons = cmd.buttons | 1;
				}
			}
		}
	}
}

impl Triggerbot {
	fn should_shoot(&self, viewangles: &sdk::QAngle) -> bool {
		let mut ptrace = sdk::trace_t::new();
		let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);

		let localplayer_entidx = IVENGINECLIENT_PTR.as_option().unwrap().get_local_player();
		let local_baseentity= ICLIENTENTITYLIST_PTR.as_option().unwrap().get_client_entity(localplayer_entidx);
		
		let me = match local_baseentity.to_option() {
			Some(ent) => ent,
			None => { log!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); libc::exit(1); }
		};

		let mut direction = sdk::Vector::new();

		sdk::angle_vectors(viewangles, &direction );
		let eyes = me.get_origin();
		
		let eye_offset_ptr = ((me as *mut IClientEntity as uint) + 0xF8) as *const [float, ..3];
		eyes.x += eye_offsets[0];
		eyes.y += eye_offsets[1];
		eyes.z += eye_offsets[2];
	
		direction = direction * 8192 + eyes;
		
		let ray = sdk::Ray_t::new(eyes, direction);

		getptr_ienginetrace().as_option().unwrap().trace_ray(ray, 0x200400B, &filter, &pTrace);
		if ( pTrace.allsolid ) {
			return false;
		}

		if ( pTrace.m_pEnt )
		{
			let entidx = pTrace.m_pEnt.as_option().unwrap().index;	
			log!("Hit entity {} at hitgroup {}", entidx, pTrace.hitgroup);
			if (pTrace.hitgroup == HITGROUP_HEAD) { //&& ((*(int *)((((char *)pTrace.m_pEnt)+0x00AC)) != (*(int *)((((char *)pBaseEntity)+0x00AC)))))) {
				return true;
			}
			return false; //pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
		}

		false;
	}
}