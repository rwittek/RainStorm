#include "SDK.h"
#include "Player ESP.h"
#include "Player Manager.h"
#include "Draw Manager.h"
#include "World Objects.h"
#include "Control Variables.h"
#include "Font Manager.h"
#include "Extra Functions.h"
#include "Aimbot.h"
#include "Client.h"
#include "Utilities.h"

//===================================================================================
CPlayerESP gPlayerESP;
//===================================================================================
void CPlayerESP::DrawPlayerESP( int iIndex )
{
	C_BaseEntity* pLocalBaseEntity = gPlayers[me].BaseEnt();

	if( pLocalBaseEntity == NULL )
		return;

	player_info_s pInfo;

	C_BaseEntity* pBaseEntity = gPlayers[iIndex].BaseEnt();

	if( pBaseEntity == NULL )
		return;

	if( iIndex == pLocalBaseEntity->index )
		return;

	if (!gCvars.esp_team && (*MakePtr( int*, pBaseEntity, gOffsets.m_iTeamNum ) == *MakePtr( int*, pLocalBaseEntity, gOffsets.m_iTeamNum )))
		return;

	if ( *MakePtr( BYTE*, pBaseEntity, gOffsets.m_lifeState) == LIFE_ALIVE && pBaseEntity->IsDormant( ) == false && g_pEngine->GetPlayerInfo( iIndex, &pInfo ) && pBaseEntity->IsPlayer( ) )
	{
		Vector vScreen, vWorldPos;

		int iTeamNum = *MakePtr( int*, pBaseEntity, gOffsets.m_iTeamNum ); 

		DWORD dwGetTeamColor = gColorManager.dwGetColor( iTeamNum );
		GetWorldSpaceCenter( pBaseEntity, vWorldPos );

		if( gDrawManager.WorldToScreen( vWorldPos, vScreen ) == false )
			return;

		float flDistance = flGetDistance( pBaseEntity->GetAbsOrigin( ), pLocalBaseEntity->GetAbsOrigin( ) );
		int iRadius = ( 300.0 * 90.0 ) / ( flDistance * 90.0 );

		if( gCvars.esp_box )
		{
			gDrawManager.DrawBox(pBaseEntity->EyePosition(), RED( dwGetTeamColor ), GREEN( dwGetTeamColor ), BLUE( dwGetTeamColor ), ALPHA( dwGetTeamColor ), 4, iRadius );
		}
		if( gCvars.esp_name )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwGetTeamColor, "%s", pInfo.name);
			vScreen.y += gFontManager.GetESPHeight( );
		}
		if( gCvars.esp_dist && *MakePtr(BYTE*,pLocalBaseEntity,gOffsets.m_lifeState) == LIFE_ALIVE)
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwGetTeamColor, "[%.0fm]", flDistance );
			vScreen.y += gFontManager.GetESPHeight( );
		}
		if( gCvars.esp_health )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwGetTeamColor, "%i HP", *MakePtr( int*, pBaseEntity, gOffsets.m_iHealth ) );
			vScreen.y += gFontManager.GetESPHeight( );
		}
		if( gCvars.esp_class )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwGetTeamColor, "%s",  szGetTF2Class(*MakePtr(int*,pBaseEntity,gOffsets.m_iPlayerClass)) );
			vScreen.y += gFontManager.GetESPHeight( );
		}
		if (gCvars.misc_spy_disg)
		{
			int* m_nPlayerCond = MakePtr(int*,pBaseEntity,gOffsets.m_Shared + gOffsets.m_nPlayerCond);
			if (*m_nPlayerCond & TFCond_Disguised) // Exclusive for spy, no need for class check
				*m_nPlayerCond &= ~TFCond_Disguised;
		}
		if (gCvars.misc_spy_cloak)
		{
			int* m_nPlayerCond = MakePtr(int*,pBaseEntity,gOffsets.m_Shared + gOffsets.m_nPlayerCond);
			if (*m_nPlayerCond & TFCond_Cloaked) // Exclusive for spy, no need for class check
				*m_nPlayerCond &= ~TFCond_Cloaked;
		}
	}

}
//===================================================================================
void CPlayerESP::DrawWorldESP( int iIndex )
{
	if( !gCvars.esp_object )
		return;

	C_BaseEntity* pLocalBaseEntity = gPlayers[me].BaseEnt();

	IClientEntity* pEntity = g_pEntList->GetClientEntity( iIndex );

	if( pEntity == NULL )
		return;

	C_BaseEntity* pBaseEntity = dynamic_cast< C_BaseEntity* >( pEntity );

	if( pBaseEntity == NULL )
		return;

	if( pBaseEntity->IsDormant( ) == false )
	{
		Vector vScreen;
		int iTeamNum;

		if ( gDrawManager.WorldToScreen( pBaseEntity->GetAbsOrigin( ), vScreen ) == false )
			return;

		DWORD dwTeamColor = gColorManager.dwGetColor( *MakePtr( int*, pBaseEntity, gOffsets.m_iTeamNum ) );

		/* Intel */
		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CCaptureFlag" ) )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "[**Capture**]" );
		}

		/* Baseball */
		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CTFStunBall" ) )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "[.]" );
		}

		/* Grenades and Pipebombs */
		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CTFGrenadePipebombProjectile" ) )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "[*]" );
		}

		/* Rockets */
		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CTFProjectile_Rocket" ) )
		{
			gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "[->]" );
			// Lol, nignogger's code is so fucking inaccurate and slow
			// 			int teamoff = gOffsets.m_iTeamNum;			
			// 			int team = *MakePtr(int*,pBaseEntity,teamoff);		
			// 			int myteam = *MakePtr(int*,pLocalBaseEntity,teamoff);		
			// 			if(myteam != team)
			// 			{
			//			Vector myhead;					
			//			myhead.x = pLocalBaseEntity->GetAbsOrigin().x;
			//			myhead.y = pLocalBaseEntity->GetAbsOrigin().y;
			//			myhead.z = pLocalBaseEntity->GetAbsOrigin().z + 74;	
			//			gAimbot.CalcAngle( myhead, pBaseEntity->GetAbsOrigin(), _pCommand->viewangles);
			//  				int _distance = (pLocalBaseEntity->GetAbsOrigin().DistTo(pBaseEntity->GetAbsOrigin()) / 22.0f);
			//   				if (_distance < 15)
			//   					_pCommand->buttons |= IN_ATTACK2;
			// 			}
		}

		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CObjectSentrygun" ) )
		{
			CObjectSentryGun* pSentryGun = reinterpret_cast< CObjectSentryGun* >( pBaseEntity );

			if( pSentryGun == NULL )
				return;

			if (!(*MakePtr(int*,pSentryGun,gOffsets.m_iUpgradeLevel)))
				return;

			if( *MakePtr( bool*, pBaseEntity, gOffsets.m_bHasSapper)  )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "**SAPPED**" );
				vScreen.y += gFontManager.GetESPHeight( );
			}

			if( gCvars.esp_object )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Sentry: %i", *MakePtr( int*, pSentryGun, gOffsets.m_iUpgradeLevel) );
				vScreen.y += gFontManager.GetESPHeight( );

				// If it was building
				if( *MakePtr(bool*,pSentryGun,gOffsets.m_bBuilding) )
				{

					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Building: %0.f%%", *MakePtr(float*,pSentryGun,gOffsets.m_flPercentageConstructed) * 100);
					vScreen.y += gFontManager.GetESPHeight( );

				}
				else 
				{
					// Normal state then
					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%i hp", *MakePtr( int*, pSentryGun, gOffsets.m_iHealth_object) );
					vScreen.y += gFontManager.GetESPHeight( );

					if(*MakePtr(int*,pSentryGun,gOffsets.m_iUpgradeLevel) < 3)
					{
						gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Upgraded: %i%%", *MakePtr(int*,pSentryGun,gOffsets.m_iUpgradeMetal) / 2 ); // (Netvar / 200) * 100 == Netvar / 2
						vScreen.y += gFontManager.GetESPHeight( );
					}

					if (*MakePtr(bool*,pSentryGun,gOffsets.m_bPlayerControlled))
						gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%s", "Controlling" );
					else 
						gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%s", pSentryGun->GetStateString( *MakePtr(int*,pSentryGun,gOffsets.m_iState) ) );

					vScreen.y += gFontManager.GetESPHeight( );

					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%i Ammo", *MakePtr(int*,pSentryGun,gOffsets.m_iAmmoShells) );
					vScreen.y += gFontManager.GetESPHeight( );
					if (*MakePtr(int*,pSentryGun,gOffsets.m_iUpgradeLevel) == 3)
					{
						gFontManager.DrawString(false,vScreen.x,vScreen.y,dwTeamColor,"%i Rockets",*MakePtr(int*,pSentryGun,gOffsets.m_iAmmoRockets));
						vScreen.y += gFontManager.GetESPHeight( );
					}

				}
			}
		}

		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CObjectDispenser" ) )
		{
			CObjectDispenser* pDispenser = reinterpret_cast< CObjectDispenser* >( pBaseEntity );

			if( pDispenser == NULL )
				return;

			if (!(*MakePtr(int*,pDispenser,gOffsets.m_iUpgradeLevel)))
				return;

			if( *MakePtr( bool*, pDispenser, gOffsets.m_bHasSapper) )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "**SAPPED**", *MakePtr( bool*, pDispenser, gOffsets.m_bHasSapper) );
				vScreen.y += gFontManager.GetESPHeight( );
			}

			if( gCvars.esp_object )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Dispenser: %i", *MakePtr( int*, pDispenser, gOffsets.m_iUpgradeLevel) );
				vScreen.y += gFontManager.GetESPHeight( );

				// If it was building
				if( *MakePtr(bool*,pDispenser,gOffsets.m_bBuilding) )
				{
					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Building: %0.f%%", *MakePtr(float*,pDispenser,gOffsets.m_flPercentageConstructed) * 100);
					vScreen.y += gFontManager.GetESPHeight( );
				}
				else 
				{
					// Normal state then
					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%i hp", *MakePtr( int*, pDispenser, gOffsets.m_iHealth_object) );
					vScreen.y += gFontManager.GetESPHeight( );

					if(*MakePtr(int*,pDispenser,gOffsets.m_iUpgradeLevel) < 3)
					{
						gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Upgraded: %i", *MakePtr(int*,pDispenser,gOffsets.m_iUpgradeMetal) / 2 );
						vScreen.y += gFontManager.GetESPHeight( );
					}
					gFontManager.DrawString(false,vScreen.x,vScreen.y,dwTeamColor,"%i Metal",*MakePtr(int*,pDispenser,gOffsets.m_iAmmoMetal));

					vScreen.y += gFontManager.GetESPHeight( );
				}
			}
		}

		if( !strcmp( pBaseEntity->GetClientClass( )->GetName( ), "CObjectTeleporter" ) )
		{
			CObjectTeleporter* pTeleporter = reinterpret_cast< CObjectTeleporter* >( pBaseEntity );

			if( pTeleporter == NULL )
				return;

			if (!(*MakePtr(int*,pTeleporter,gOffsets.m_iUpgradeLevel)))
				return;

			if( *MakePtr( bool*, pTeleporter, gOffsets.m_bHasSapper) )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "**SAPPED**", *MakePtr( bool*, pTeleporter, gOffsets.m_bHasSapper) );
				vScreen.y += gFontManager.GetESPHeight( );
			}

			if( gCvars.esp_object )
			{
				gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Teleporter: %i", *MakePtr( int*, pTeleporter, gOffsets.m_iUpgradeLevel) );
				vScreen.y += gFontManager.GetESPHeight( );

				// If it was building
				if( *MakePtr(bool*,pTeleporter,gOffsets.m_bBuilding) )
				{
					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Building: %0.f%%", *MakePtr(float*,pTeleporter,gOffsets.m_flPercentageConstructed) * 100);
					vScreen.y += gFontManager.GetESPHeight( );
				}
				else 
				{
					// Normal state then
					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%i hp", *MakePtr( int*, pTeleporter, gOffsets.m_iHealth_object) );
					vScreen.y += gFontManager.GetESPHeight( );

					if(*MakePtr(int*,pTeleporter,gOffsets.m_iUpgradeLevel) < 3)
					{
						gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "Upgraded: %i", *MakePtr(int*,pTeleporter,gOffsets.m_iUpgradeMetal) / 2 );
						vScreen.y += gFontManager.GetESPHeight( );
					}

					gFontManager.DrawString( false, vScreen.x, vScreen.y, dwTeamColor, "%s", pTeleporter->GetStateString( *MakePtr(int*,pTeleporter,gOffsets.m_iState) ) );

					vScreen.y += gFontManager.GetESPHeight( );
				}
			}
		}
	}
}
