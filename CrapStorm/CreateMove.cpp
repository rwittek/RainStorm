#include "CreateMove.h"
#include "Player Manager.h"
#include "Client.h"
#include "Extra Functions.h"
#include "Control Variables.h"
#include "Player ESP.h"
#include "Aimbot.h"
#include "Utilities.h"

void CCreateMove::Invoke()
{	
	C_BaseEntity* pLocalBaseEntity = gPlayers[me].BaseEnt();

	if( pLocalBaseEntity == NULL )
		return;

	C_BaseCombatWeapon *pBaseWeapon = GetBaseCombatActiveWeapon(pLocalBaseEntity);

	if( pBaseWeapon == NULL )
		return;
#if COMPILE_AIMBOT
	gAimbot.FindTarget( );
	gAimbot.AimAtTarget( );
#endif

#if COMPILE_MISC
	if (!Q_strcmp(GetWeaponClass(pBaseWeapon),"CTFKnife") && gCvars.misc_auto_bs)
	{
		if (*(MakePtr( bool*, pBaseWeapon, gOffsets.m_bReadyToBackstab ))) // It was slow because it was 1 tick behind and netvar was also 1 tick behind too
			_pCommand->buttons |= IN_ATTACK;
	}
	if( gCvars.misc_bunnyhop && _pCommand->buttons & IN_JUMP )
	{
		int iFlags = pLocalBaseEntity->GetFlags( );

		if( !(iFlags & FL_ONGROUND) )
			_pCommand->buttons &= ~IN_JUMP;
	}
#endif
	// Put your code below
}