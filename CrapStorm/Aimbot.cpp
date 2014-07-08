#if COMPILE_AIMBOT
#include "Aimbot.h"
#include "Control Variables.h"
#include "Player Manager.h"
#include "Draw Manager.h"
#include "Extra Functions.h"
#include "Utilities.h"
#include "Client.h"

CAimbot gAimbot;

//===================================================================================
bool bIsAimbotActive( int iAimValue )
{
	if ( iAimValue == 0 )
	{
		return false;
	}
	if ( iAimValue == 1  && GetAsyncKeyState( VK_LBUTTON ) )
	{
		return true;
	}

	return false;
}
//===================================================================================
CAimbot::CAimbot( void )
{
	m_nTarget = -1;
}
//===================================================================================
bool CAimbot::bIsTargetSpot( int iIndex )
{
	C_BaseEntity* pBaseEntity = gPlayers[iIndex].BaseEnt( );

	if( pBaseEntity == NULL )
		return false;

	if( pBaseEntity->IsDormant( ) )
		return false;

	C_BaseCombatCharacter* pBaseCombat = gPlayers[iIndex].CombatCharacter( );

	if( pBaseCombat == NULL )
		return false;

	Vector vForward, vRight, vUp;
	QAngle qPlayerAngle( 0, gPlayers[iIndex].BaseEnt( )->EyeAngles( ).y, 0 );

	AngleVectors( qPlayerAngle, &vForward, &vRight, &vUp );

	vForward.z = -vForward.z;

	vTargetOrg = gPlayers[iIndex].BaseEnt( )->WorldSpaceCenter( ) + ( vForward + vUp + vRight );

	PredictTarget( vTargetOrg, vPrediction );

	return true;

	return false;
}
//===================================================================================
void MakeVector( const QAngle &qAngle, QAngle& qVector ) 
{ 
	float pitch; 
	float yaw; 
	float tmp;           

	pitch	= (float)( qAngle[0] * M_PI/180.0f ); 
	yaw		= (float)( qAngle[1] * M_PI/180.0f ); 
	tmp		= (float) cos(pitch);

	qVector[0] = (float) ( -tmp * -cos( yaw ) ); 
	qVector[1] = (float) ( sin(yaw) * tmp );
	qVector[2] = (float) -sin( pitch );
}
//===================================================================================
float CAimbot::CalcFOVAngle( QAngle qAngle, Vector vSrc, Vector vDst ) 
{
	float fFOV = 0.0f; 
	QAngle qAng,qAim; 

	CalcAngle( vSrc, vDst, qAng ); 
	MakeVector( qAngle, qAim ); 
	MakeVector( qAng, qAng );      

	float mag_s = sqrt(SQUARE(qAim[0]) + SQUARE(qAim[1]) + SQUARE(qAim[2])); 
	float mag_d = sqrt(SQUARE(qAng[0]) + SQUARE(qAng[1]) + SQUARE(qAng[2])); 

	float u_dot_v = qAim[0]*qAng[0] + qAim[1]*qAng[1] + qAim[2]*qAng[2]; 

	fFOV = acos(u_dot_v / (mag_s*mag_d)) * (180.0 / M_PI); 

	return fFOV; 
}
//===================================================================================
bool CAimbot::bIsValidEntity( int iIndex )
{
	if( gPlayers[iIndex].Ent( ) == NULL || gPlayers[iIndex].BaseEnt( ) == NULL )
		return false;

	if ( gPlayers[me].Ent( ) == NULL || gPlayers[me].BaseEnt( ) == NULL )
		return false;

	if( iIndex == g_pEngine->GetLocalPlayer( ) )
		return false;

	if( gPlayers[iIndex].BaseEnt( )->IsDormant( ) )
		return false;

	// 	if( gPlayers[iIndex].BaseEnt( )->IsSolid( ) == false )
	// 		return false;

	if( gPlayers[iIndex].BaseEnt( )->IsPlayer( ) == false )
		return false;

	if( gPlayers[iIndex].BaseEnt( )->IsAlive( ) == false || (*MakePtr(byte*,gPlayers[iIndex].BaseEnt(),gOffsets.m_lifeState) != LIFE_ALIVE))
		return false;

	return true;
}
//===================================================================================
bool CAimbot::bIsValidTarget( int iIndex )
{
	if( bIsValidEntity( iIndex ) == false )
		return false;
	if( *MakePtr(int*,gPlayers[iIndex].BaseEnt(),gOffsets.m_iTeamNum) == *MakePtr(int*,gPlayers[me].BaseEnt(),gOffsets.m_iTeamNum) )
		return false;
	if( bIsTargetSpot( iIndex ) == false )
		return false;
	if( CalcFOVAngle( gPlayers[me].BaseEnt( )->EyeAngles( ), gPlayers[me].BaseEnt( )->EyePosition( ) , vPrediction ) > gCvars.aim_fov )
		return false;
	return bIsVisible( gPlayers[me].BaseEnt( )->EyePosition( ), vPrediction, gPlayers[iIndex].BaseEnt( ) );
}
//===================================================================================
float CAimbot::flGetDistance( Vector vOrigin )
{
	IClientEntity *pLocalEnt = g_pEntList->GetClientEntity( g_pEngine->GetLocalPlayer( ) );

	if( pLocalEnt == NULL )
		return 1.0f;

	Vector vDistance = vOrigin - pLocalEnt->GetAbsOrigin( );

	float flDistance = FastSqrt( vDistance.Length( ) );

	if( flDistance < 1.0f )
		return 1.0f;

	return flDistance;
}
//===================================================================================
float CAimbot::flGetFOV( Vector vOrigin ) 
{ 
	float fFOV = 0.0f; 
	QAngle qAngle, qAim;

	C_BaseEntity* pBaseEnt = gPlayers[me].BaseEnt();

	if( pBaseEnt == NULL )
		return 1.0f;

	CalcAngle( pBaseEnt->EyePosition( ), vOrigin, qAngle ); 
	MakeVector( pBaseEnt->EyeAngles( ), qAim ); 
	MakeVector( qAngle, qAngle );      

	float mag_s = FastSqrt( qAim.Length( ) ); 
	float mag_d = FastSqrt( qAngle.Length( ) ); 

	float u_dot_v = qAim[0]*qAngle[0] + qAim[1]*qAngle[1] + qAim[2]*qAngle[2]; 

	fFOV = acos(u_dot_v / (mag_s*mag_d)) * (180.0f / M_PI); 

	return fFOV; 
}
//===================================================================================
void CAimbot::FindTarget( void )
{
	if( gCvars.aim_lock && m_nTarget != -1 && bIsValidEntity( m_nTarget ) && bIsValidTarget( m_nTarget ) )
	{
		return;
	}

	
	DropTarget( );

	for( int iIndex = 0; iIndex <= g_pEngine->GetMaxClients( ) + 1; iIndex++ )
	{
		if( bIsValidTarget( iIndex ) )
		{
				if ( flBestDist < flGetDistance( vPrediction ) )
					return;

				flBestDist = flGetDistance( vPrediction );
				m_nTarget = iIndex;
				gPlayers[iIndex].SetAimOrg( vPrediction );
		}
	}
}
//===================================================================================
bool CAimbot::bIsVisible( const Vector& vecAbsStart, const Vector& vecAbsEnd, C_BaseEntity* pBaseEnt ) 
{
	player_info_t pInfo;

	trace_t pTrace;
	Ray_t pRay; 

	pRay.Init( vecAbsStart, vecAbsEnd ); 

	//g_pEngineTrace->TraceRay( pRay, MASK_NPCWORLDSTATIC, NULL, &pTrace );
	g_pEngineTrace->TraceRay( pRay, ( CONTENTS_SOLID|CONTENTS_MOVEABLE|CONTENTS_MONSTER|CONTENTS_HITBOX ), NULL, &pTrace);

	if( pTrace.m_pEnt == NULL )
		return false;

	if ( pTrace.allsolid )
		return false;

	if( !strcmp( pTrace.m_pEnt->GetClientClass( )->GetName( ), "CFuncRespawnRoomVisualizer" ) )
		return true;

	if( pTrace.m_pEnt->IsPlayer( ) )
	{
		if ( g_pEngine->GetPlayerInfo( pTrace.m_pEnt->index, &pInfo ) && pBaseEnt == pTrace.m_pEnt )
		{
			if( pTrace.fraction >= 0.92f || pTrace.DidHitNonWorldEntity( ) )
			{
				if ( pTrace.hitgroup >= HITGROUP_HEAD || pTrace.hitgroup <= HITGROUP_RIGHTLEG )
					return true;
			}
		}
	}

	return false;
}
//===================================================================================
int CAimbot::iGetTarget( void )
{
	return m_nTarget;
}
//===================================================================================
void CAimbot::PredictTarget( Vector &vOrigin, Vector &vPredicted )
{
	if( gCvars.aim_prediction )
	{
		static Vector vOldOrigin( 0, 0, 0 );
		static Vector vOldestOrigin( 0, 0, 0 );
		Vector vDeltaOrigin(0, 0, 0);

		// Calculate the delta origin
		vDeltaOrigin = vOrigin - vOldestOrigin;
		vOldestOrigin = vOldOrigin;
		vOldOrigin = vOrigin;

		// Get the latency
		float flLatency = g_pEngine->GetNetChannelInfo( )->GetLatency( FLOW_OUTGOING );

		// Compensate the latency
		vDeltaOrigin[0] *= flLatency;
		vDeltaOrigin[1] *= flLatency;
		vDeltaOrigin[2] *= flLatency;

		// Apply the prediction
		vPredicted = vOrigin + vDeltaOrigin;
	}
	else
		VectorCopy( vOrigin, vPredicted );
}
//===================================================================================
void CAimbot::CalcAngle( const Vector& vSource, const Vector& vDest, QAngle& vAngles )
{
	double delta[3] = { (vSource[0]-vDest[0]), (vSource[1]-vDest[1]), (vSource[2]-vDest[2]) };
	double hyp = sqrt( delta[0]*delta[0] + delta[1]*delta[1] );

	vAngles[0] = (float) (atan(delta[2]/hyp) * M_RADPI);
	vAngles[1] = (float) (atan(delta[1]/delta[0]) * M_RADPI);
	vAngles[2] = 0.0f;

	if( delta[0] >= 0.0f ) 
		vAngles[1] += 180.0f;
}
//===================================================================================
void CAimbot::AimAtTarget() //when prone they're dormant
{
	bool bAiming = bIsAimbotActive( gCvars.aim_key );

	if( gCvars.aim_auto_aim )
		bAiming = true;

	if( g_pEngine->IsInGame( ) == false || g_pEngine->IsHLTV( ) == true || g_pEngine->Con_IsVisible( ) || !gCvars.aim_bot || bAiming == false )
		return;

	if( gPlayers[me].Ent( ) == NULL || gPlayers[me].BaseEnt( ) == NULL )
		return;

	if( m_nTarget == -1 )
		return;

	CUserCmd* oldCommand = _pCommand;
	QAngle oldangle = oldCommand->viewangles;

	CalcAngle( gPlayers[me].BaseEnt( )->EyePosition( ), gPlayers[m_nTarget].GetAimOrg( ), qAimAngles );

	_pCommand->viewangles = qAimAngles;
	
	if (_pCommand != oldCommand)
		_pCommand->viewangles = oldangle;
	//g_pEngine->SetViewAngles( qAimAngles );
	if (gCvars.aim_auto_shoot)
		_pCommand->buttons |= IN_ATTACK;
}
//===================================================================================
void CAimbot::DropTarget( void )
{
	m_nTarget = -1;
	flBestDist = 9999.0f;
}
//===================================================================================
bool CGameTrace::DidHitWorld() const
{
	return m_pEnt == g_pEntList->GetClientEntity( 0 );
}
//===================================================================================
bool CGameTrace::DidHitNonWorldEntity() const
{
	return m_pEnt != g_pEntList->GetClientEntity( 0 );
}
//===================================================================================
#endif