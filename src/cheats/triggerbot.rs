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
		sdk::trace_t pTrace;
		sdk::Ray_t pRay;
		let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);

		IClientEntity* pBaseEntity = (getptr_icliententitylist()->GetClientEntity((rainstorm_getivengineclient()->GetLocalPlayer())));;

		let me: match pBaseEntity.to_option() {
			Some(ent) -> ent,
			None -> { log!("IClientEntity of local player (id: {}) not found!\n", IVENGINECLIENT_PTR->GetLocalPlayer()); libc::exit(1); }
		}

		sdk::Vector vDirection;

		sdk::angle_vectors(viewangles, &vDirection );
		sdk::Vector eyes = me.get_origin();
		
		let eye_offset_ptr = ((me as *mut IClientEntity as uint) + 0xF8) as *const [float, ..3];
		eyes.x += eye_offsets[0];
		eyes.y += eye_offsets[1];
		eyes.z += eye_offsets[2];
	
		vDirection = vDirection * 8192 + eyes;
		
		pRay.Init( eyes, vDirection );

		getptr_ienginetrace()->TraceRay(pRay, ( CONTENTS_SOLID|CONTENTS_MOVEABLE|CONTENTS_MONSTER|CONTENTS_DEBRIS|CONTENTS_HITBOX ), filter.itracefilter, &pTrace);
		if ( pTrace.allsolid )
			return false;

		if ( pTrace.m_pEnt )
		{
			int entidx = pTrace.m_pEnt->index;	
			fprintf(logfile, "%d\n", entidx);
			fprintf(logfile, "%d\n", pTrace.hitgroup);
			if (filter.hit_player && pTrace.hitgroup == HITGROUP_HEAD && ((*(int *)((((char *)pTrace.m_pEnt)+0x00AC)) != (*(int *)((((char *)pBaseEntity)+0x00AC)))))) {
				return true;
			}
			if ( getptr_ivengineclient()->GetPlayerInfo( pTrace.m_pEnt->index, &pInfo ) == false )
				return false;

			//return true;
			return false; //pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
		}

		return false;
	}
}