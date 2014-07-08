#include "SDK.h"
#include "Extra Functions.h"
#include "Client.h"
#include "Player Manager.h"
//===================================================================================
CVerifyFuncs gVerifyFuncs;
//===================================================================================
int	C_BasePlayer::GetUserID( void )
{
	player_info_t pi;

	if ( !g_pEngine->GetPlayerInfo( g_pEngine->GetLocalPlayer( ), &pi ) )
		return -1;

	return pi.userID;
}
//===================================================================================
void Normalize(Vector &vIn, Vector &vOut)
{
	float flLen = vIn.Length();

	if(flLen == 0)
	{
		vOut.Init(0, 0, 1);
		return;
	}

	flLen = 1 / flLen;

	vOut.Init(vIn.x * flLen, vIn.y * flLen, vIn.z * flLen);
}
//===================================================================================
void Spinbot( CUserCmd* pCommand )
{
	if( pCommand->buttons & IN_ATTACK )
		return;

	Vector viewforward, viewright, viewup, aimforward, aimright, aimup;
	QAngle qAimAngles;

	float forward = pCommand->forwardmove;
	float right = pCommand->sidemove;
	float up = pCommand->upmove;

	qAimAngles.Init( 0.0f, pCommand->viewangles.y, 0.0f );
	AngleVectors(qAimAngles, &viewforward, &viewright, &viewup);

	float fTime = g_pEngine->Time( );
	pCommand->viewangles.y = (vec_t)( fmod(fTime / 0.1f * 360.0f, 360.0f));
	qAimAngles.Init(0.0f, pCommand->viewangles.y, 0.0f);

	AngleVectors(qAimAngles, &aimforward, &aimright, &aimup);

	Vector vForwardNorm;		
	Normalize ( viewforward, vForwardNorm );
	Vector vRightNorm;			
	Normalize( viewright, vRightNorm );
	Vector vUpNorm;				
	Normalize( viewup, vUpNorm );

	pCommand->forwardmove = DotProduct(forward * vForwardNorm, aimforward) + DotProduct(right * vRightNorm, aimforward) + DotProduct(up * vUpNorm, aimforward);
	pCommand->sidemove = DotProduct(forward * vForwardNorm, aimright) + DotProduct(right * vRightNorm, aimright) + DotProduct(up * vUpNorm, aimright);
	pCommand->upmove = DotProduct(forward * vForwardNorm, aimup) + DotProduct(right * vRightNorm, aimup) + DotProduct(up * vUpNorm, aimup);
}
//===================================================================================
void GetWorldSpaceCenter( C_BaseEntity* pBaseEnt, Vector& vWorldSpaceCenter )
{
	if ( pBaseEnt )
	{
		Vector vMin, vMax;
		pBaseEnt->GetRenderBounds( vMin, vMax );
		vWorldSpaceCenter = pBaseEnt->GetAbsOrigin( );
		vWorldSpaceCenter.z += (vMin.z + vMax.z) / 2.0f;
	}
}
//===================================================================================
float flGetDistance( Vector vOrigin, Vector vLocalOrigin )
{
	Vector vDelta = vOrigin - vLocalOrigin;

	float m_fDistance = FastSqrt( vDelta.Length( ) );

	if( m_fDistance < 1.0f )
		return 1.0f;

	return m_fDistance;
}
//===================================================================================
QAngle qGetPunchAngle( void )
{
	C_BaseEntity* pBaseEntity = g_pEntList->GetClientEntity( g_pEngine->GetLocalPlayer( ) )->GetBaseEntity( );
	CSDKPlayer *pSDKPlayer = ToSDKPlayer( pBaseEntity );
	QAngle pPunchAngle = pSDKPlayer->m_Local.m_vecPunchAngle.Get( );
	return pPunchAngle;
}
//===================================================================================
const char* szGetTF2Class( int iClass)
{
	switch(iClass)
	{
		case TFClass_Scout: return "Scout";
		case TFClass_Soldier: return "Soldier";
		case TFClass_Pyro: return  "Pyro";
		case TFClass_DemoMan: return "Demoman";
		case TFClass_Heavy: /*return "gir489"*/ return "Heavy";
		case TFClass_Engineer: /*return "lmaobox"*/ return "Engineer";
		case TFClass_Medic: return "Medic";
		case TFClass_Sniper: return "Sniper";
		case TFClass_Spy: return "Spy";
	}
	return null;
}
//===================================================================================
void UnprotectCvars( void )
{
	ConCommand *pVar = (ConCommand*)g_pCvar->GetCommands( );
	
	//ConVar *pConsistency = g_pCvar->FindVar( "sv_consistency" );
	//ConVar *pCheats = g_pCvar->FindVar( "sv_cheats" );

	while( pVar )
	{
		if( pVar->IsFlagSet( FCVAR_CHEAT ) )
			pVar->m_nFlags &= ~FCVAR_CHEAT;

// 		if( pVar->IsFlagSet( FCVAR_REPLICATED ) )
// 			pVar->m_nFlags &= ~FCVAR_REPLICATED;
// 
// 		if( pVar->IsFlagSet( FCVAR_PROTECTED ) )
// 			pVar->m_nFlags &= ~FCVAR_PROTECTED;
// 
// 		if( pVar->IsFlagSet( FCVAR_SPONLY ) )
// 			pVar->m_nFlags &= ~FCVAR_SPONLY;

		pVar = (ConCommand*)pVar->GetNext( );
	}

	//pConsistency->SetValue( 0 );
	//pCheats->SetValue( 1 );
}
//===================================================================================
double dblGetPlatRealTime( void )
{
	bool bIsBench = Plat_IsInBenchmarkMode( );

	Plat_SetBenchmarkMode( false );

	double dblCurTime = Plat_FloatTime( );

	if( bIsBench )
		Plat_SetBenchmarkMode( true );

	return dblCurTime;
}
//===================================================================================
C_BaseCombatWeapon * GetBaseCombatActiveWeapon ( C_BaseEntity* pEntity )
{
	EHANDLE hActiveWeapon = (EHANDLE)*(MakePtr( int* , pEntity, gOffsets.m_hActiveWeapon ));
	return dynamic_cast< C_BaseCombatWeapon* >(g_pEntList->GetClientEntityFromHandle( hActiveWeapon ));
}
//===================================================================================
const char* GetWeaponClass( C_BaseCombatWeapon* pWeapon )
{
	return pWeapon->GetClientClass()->GetName(); 
}
//===================================================================================
bool bTraceToPlayer( void )
{
	trace_t pTrace;
	Ray_t pRay;
	player_info_t pInfo;

	C_BaseEntity* pBaseEntity = gPlayers[1337].BaseEnt();

	if ( !pBaseEntity )
		return false;

	Vector vDirection;

	AngleVectors( pBaseEntity->GetAbsAngles( ), &vDirection );

	vDirection = vDirection * 8192 + pBaseEntity->EyePosition( );
	Vector vLocalPosition = pBaseEntity->EyePosition( );

	pRay.Init( vLocalPosition, vDirection );

	g_pEngineTrace->TraceRay( pRay, ( CONTENTS_SOLID|CONTENTS_MOVEABLE|CONTENTS_MONSTER|CONTENTS_DEBRIS|CONTENTS_HITBOX ), NULL, &pTrace);

	if ( pTrace.allsolid )
		return false;

	if ( pTrace.m_pEnt )
	{
		if ( g_pEngine->GetPlayerInfo( pTrace.m_pEnt->index, &pInfo ) == false )
			return false;

		return pTrace.m_pEnt->m_iTeamNum != pBaseEntity->m_iTeamNum; // Avoid teammates.
	}

	return false;
}