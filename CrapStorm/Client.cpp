#include "SDK.h"
#include "Client.h"
#include "Panels.h"
#include "Utilities.h"
#include "Control Variables.h"
#include "Cheat Menu.h"
#include "Font Manager.h"
#include "Draw Manager.h"
#include "File Manager.h"
#include "Aimbot.h"
#include "Player Manager.h"
#include "CreateMove.h"
//===================================================================================
CClient gClient;
COffsets gOffsets;
static CDrawLayer gDrawLayer;
CUserCmd* _pCommand;
CCreateMove gCreateMove;
//===================================================================================
void DumpTable(RecvTable *pTable,FILE* fp);
void DumpOffset(char* file)
{
	FILE *fp;
	fp = fopen ( file , "a+");

	ClientClass *pClass = g_pClient->GetAllClasses();

	for( ; pClass; pClass = pClass->m_pNext )
	{
		RecvTable *pTable = pClass->m_pRecvTable;

		fprintf(fp, "-- [ %s | [%i] ]\n", pClass->GetName(),pTable->GetNumProps());

		for(int i = 0; i < pTable->GetNumProps(); i++)
		{
			RecvProp *pProp = pTable->GetProp( i );

			if( !pProp ) continue;
			fprintf(fp," -- > %s [0x%.4X]\n",pProp->GetName(),pProp->GetOffset());

			if (pProp->GetDataTable())
			{
				DumpTable(pProp->GetDataTable(),fp);
			}
		}
		fprintf(fp,"-- END [ %s | [%i] ]\n",pClass->GetName(),pTable->GetNumProps());
	}
	fclose(fp);
}
void DumpTable(RecvTable *pTable,FILE* fp)
{
	fprintf(fp, "	-- SUB [ %s | [%i] ]\n",pTable->GetName(),pTable->GetNumProps());

	for(int i = 0; i < pTable->GetNumProps(); i++)
	{
		RecvProp *pProp = pTable->GetProp( i );

		if( !pProp ) continue;
		fprintf(fp,"		-- -- > %s [0x%.4X]\n",pProp->GetName(),pProp->GetOffset());
		if (pProp->GetDataTable())
			DumpTable(pProp->GetDataTable(),fp);
	}
	fprintf(fp, "	-- END SUB [ %s | [%i] ]\n",pTable->GetName(),pTable->GetNumProps());
}
//===================================================================================
// Currently it was 2 level deep, it was enough.
int getOffset( char *szClassName, char *szVariable )
{
	ClientClass *pClass = g_pClient->GetAllClasses();

	for( ; pClass; pClass = pClass->m_pNext )
	{
		RecvTable *pTable = pClass->m_pRecvTable;

		if( pTable->GetNumProps() <= 1 ) continue;

		for(int i = 0; i < pTable->GetNumProps(); i++)
		{
			RecvProp *pProp = pTable->GetProp( i );

			if( !pProp ) continue;

			if( !Q_strcmp( pTable->GetName(), szClassName ) && !Q_strcmp( pProp->GetName(), szVariable ) )
			{
				#if DEBUG
				gBaseAPI.LogToConsole("Found Offset --> [%s --> 0x%.4X [%s]",pProp->GetName(),pProp->GetOffset(),pTable->GetName());
				#endif
				return pProp->GetOffset();
			}
			if (pProp->GetDataTable())
			{
				RecvTable *pTable = pProp->GetDataTable();
				for(int i = 0; i < pTable->GetNumProps(); i++)
				{
					RecvProp *pProp = pTable->GetProp( i );

					if( !pProp ) continue;

					if(!Q_strcmp( pTable->GetName(), szClassName ) && !Q_strcmp( pProp->GetName(), szVariable ) )
					{
						#if DEBUG
							gBaseAPI.LogToConsole("Found Offset --> [%s --> 0x%.4X] [%s]",pProp->GetName(),pProp->GetOffset(),pTable->GetName());
						#endif
						return pProp->GetOffset();
					}
				}
			}
		} 
	}
	return 0;
}
//===================================================================================
void findOffsets()
{
	// CBaseCombatCharacter
	gOffsets.m_hActiveWeapon	= getOffset("DT_BaseCombatCharacter", "m_hActiveWeapon");
	gOffsets.m_bReadyToBackstab = getOffset("DT_TFWeaponKnife", "m_bReadyToBackstab");

	// CTFPlayer
	gOffsets.m_iHealth			= getOffset("DT_BasePlayer", "m_iHealth");
	gOffsets.m_lifeState		= getOffset("DT_BasePlayer", "m_lifeState");
	gOffsets.m_iTeamNum			= getOffset("DT_BaseEntity", "m_iTeamNum");
	gOffsets.m_iPlayerClass		= getOffset("DT_TFPlayer", "m_PlayerClass") + 0x04; //m_iClass
	gOffsets.m_Shared			= getOffset("DT_TFPlayer","m_Shared");
	/* m_Shared subclass */
	gOffsets.m_nPlayerCond		= getOffset("DT_TFPlayerShared","m_nPlayerCond");

	// CBaseObject
	gOffsets.m_bHasSapper		= getOffset("DT_BaseObject", "m_bHasSapper");
	gOffsets.m_iHealth_object	= getOffset("DT_BaseObject", "m_iHealth");
	gOffsets.m_iUpgradeLevel	= getOffset("DT_BaseObject", "m_iUpgradeLevel");
	gOffsets.m_iUpgradeMetal	= getOffset("DT_BaseObject", "m_iUpgradeMetal");
	gOffsets.m_bBuilding		= getOffset("DT_BaseObject","m_bBuilding");
	gOffsets.m_flPercentageConstructed	= getOffset("DT_BaseObject", "m_flPercentageConstructed");

	// CObjectSentrygun
	gOffsets.m_iAmmoShells		= getOffset("DT_ObjectSentrygun","m_iAmmoShells");
	gOffsets.m_iState		    = getOffset("DT_ObjectSentrygun","m_iState"); // Actually this offset was universal
	gOffsets.m_bPlayerControlled = getOffset("DT_ObjectSentrygun","m_bPlayerControlled");
	gOffsets.m_iAmmoRockets     =  getOffset("DT_ObjectSentrygun","m_iAmmoRockets");
	
	// CObjectDispenser
	gOffsets.m_iAmmoMetal		= getOffset("DT_ObjectDispenser","m_iAmmoMetal");

}
//===================================================================================
int __stdcall Hooked_Init( CreateInterfaceFn appSysFactory, CreateInterfaceFn physicsFactory, CGlobalVarsBase* pGlobals )
{
	g_AppSysFactory = appSysFactory;
	XASSERT( g_AppSysFactory );

	g_pGlobals = pGlobals;
	XASSERT( g_pGlobals );

	g_pCvar = ( ICvar* ) g_AppSysFactory( CVAR_INTERFACE_VERSION, NULL );
	XASSERT( g_pCvar );

#if DEBUG
	gBaseAPI.LogToFile( "g_AppSysFactory: [0x%.8X]", (DWORD)g_AppSysFactory );
	gBaseAPI.LogToFile( "g_pGlobals: [0x%.8X]", (DWORD)g_pGlobals );
	gBaseAPI.LogToFile( "g_pCvar: [0x%.8X]", (DWORD)g_pCvar );
	gBaseAPI.BuildDebugConsole();
#endif

	//this is from hudupdate
	MathLib_Init( 2.2f, 2.2f, 0.0f, 2.0f, true, true, true, true );

	ConnectTier1Libraries( &g_AppSysFactory, 1 );
	ConnectTier2Libraries( &g_AppSysFactory, 1 );
	ConnectTier3Libraries( &g_AppSysFactory, 1 );
	//====

	// Grabbing the screen size
	g_pEngine->GetScreenSize( gScreenSize.iScreenWidth, gScreenSize.iScreenHeight );
	gFontManager.Initialize( );

	gFileManager.Initialize( gBaseAPI.GetDirectoryFile( "Darkstorm.ini" ) );
	gColorManager.Initialize( );

	gCvars.Initialize( );
	//====
	gCvars.Load( );
	ConVar_Register(0);

	gBaseAPI.LogToFile("Injection Successful");
	findOffsets();
	//DumpOffset("full directory path,eg C:\\netvar.log");
	return gClient.Init( appSysFactory, physicsFactory, pGlobals );
}
//===================================================================================
void __stdcall Hooked_CreateMove( int sequence_number, float input_sample_frametime, bool active )
{
	gClient.CreateMove( sequence_number, input_sample_frametime, active );
	gCheatMenu.Render();
	if( g_pInput == NULL )
	{
		//gAimbot.AimAtTarget( NULL );
	}
	else
	{
		if( g_pEngine->IsLevelMainMenuBackground( ) || g_pEngine->IsDrawingLoadingImage( ) || g_pEngine->IsInGame( ) == false )
			return;

		CUserCmd* pCommand = g_pInput->GetUserCmd( sequence_number );
		_pCommand = pCommand;
		gCreateMove.Invoke();
		CVerifiedUserCmd *pSafeCommand = *reinterpret_cast<CVerifiedUserCmd**>((size_t)g_pInput + 0xC8) + (sequence_number%MULTIPLAYER_BACKUP);
		pSafeCommand->m_cmd = *pCommand;
		pSafeCommand->m_crc = pSafeCommand->m_cmd.GetChecksum();
	}
}
//===================================================================================
int __stdcall Hooked_IN_KeyEvent ( int eventcode, int keynum, const char *pszCurrentBinding )
{
	if(eventcode == 1)
	{
		if( keynum == 72 ) //insert
		{
			gCheatMenu.bMenuActive = !gCheatMenu.bMenuActive;
			gCvars.Save( );
		}
		
		if(gCheatMenu.bMenuActive)
		{
			if(keynum == 88 || keynum == 112) // Up
			{
				
				if( gCheatMenu.iMenuIndex > 0 ) gCheatMenu.iMenuIndex--;
				else gCheatMenu.iMenuIndex = gCheatMenu.iMenuItems - 1;
				return 0;
				
			}
			else if(keynum == 90 || keynum == 113 ) // Down
			{
				
				if( gCheatMenu.iMenuIndex < gCheatMenu.iMenuItems - 1 ) gCheatMenu.iMenuIndex++;
				else gCheatMenu.iMenuIndex = 0;
				return 0;
				
			}
			else if(keynum == 89 || keynum == 107 ) // Left
			{
				
				if( gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value )
				{
					gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] -= gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flStep;
					if( gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] < gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flMin )
						gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] = gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flMax;
				}
				return 0;
				
			}
			else if(keynum == 91 || keynum == 108 ) // Right
			{
				
				if( gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value )
				{
					gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] += gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flStep;
					if( gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] > gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flMax )
						gCheatMenu.pMenu[gCheatMenu.iMenuIndex].value[0] = gCheatMenu.pMenu[gCheatMenu.iMenuIndex].flMin;
				}
				return 0;
				
			}
			
		}

		
		/*if( g_pEngine->IsInGame( ) && gCheatMenu.bMenuActive == false ) 
		{
			if( gCvars.misc_speed_on && keynum == 107 && eventcode && !gCvars.misc_speed_mode )
				return 0;
			if( gCvars.misc_speed_on && keynum == 108 && eventcode && !gCvars.misc_speed_mode )
				return 0;
			if( gCvars.misc_speed_on && keynum == 109 && eventcode && !gCvars.misc_speed_mode )
				return 0;
			if( gCvars.misc_speed_on && keynum == 16 && eventcode && !gCvars.misc_speed_mode )
				return 0;
			if( gCvars.misc_speed_on && keynum == 79 && eventcode && !gCvars.misc_speed_mode )
				return 0;
		}*/
	}
	return gClient.IN_KeyEvent( eventcode, keynum, pszCurrentBinding );
}
//===================================================================================