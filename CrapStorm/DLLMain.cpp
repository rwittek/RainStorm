#include "SDK.h"
#include "VMT Hook.h"
#include "Utilities.h"
#include "Client.h"
#include "Panels.h"
#include "Memory Tools.h"
#include "Extra Functions.h"

//===================================================================================
HMODULE hmThisModule = NULL;
DWORD dwSaveUserCmd0 = NULL;
DWORD dwSaveUserCmd1 = NULL;
//===================================================================================
// Base Interfaces
//===================================================================================
IBaseClientDLL*					g_pClient = NULL;
vgui::ISurface*					g_pSurface = NULL;
IVEngineClient*					g_pEngine = NULL;
IEngineVGui*					g_pEngineVGUI = NULL;
IClientEntityList*				g_pEntList = NULL;
CInput*							g_pInput = NULL;
ICvar*					        g_pCvar = NULL;
IEngineTrace*				    g_pEngineTrace = NULL;
CGlobalVarsBase*			    g_pGlobals = NULL;
//IGameEventManager2*			g_pGameEventManager = NULL;
//IPhysicsSurfaceProps*			g_pPhysicsAPI = NULL;
//IPhysics*						g_pPhysics = NULL;
IPanel*							g_pIPanel = NULL;
//CUserMessages*				g_pUserMessages = NULL;
IPrediction*					g_pPrediction = NULL;
IUniformRandomStream*			g_pRandom = NULL;
IVModelInfoClient*			    g_pModelInfo = NULL;
IVRenderView*					g_pRenderView		  = NULL;
//===================================================================================
// Factories
//===================================================================================
CreateInterfaceFn				g_ClientFactory = NULL;
CreateInterfaceFn				g_EngineFactory = NULL;
CreateInterfaceFn				g_AppSysFactory = NULL;
CreateInterfaceFn				g_VGUIFactory = NULL;
CreateInterfaceFn				g_VGUI2Factory = NULL; //new
CreateInterfaceFn				g_PhysicsFactory = NULL;
CreateInterfaceFn				g_MaterialFactory = NULL;
CreateInterfaceFn				g_FileSystemFactory = NULL;
//===================================================================================
CVMTHookManager*				g_pClientHook = NULL;
//===================================================================================


//===================================================================================
// Steam Interfaces & Factory
//===================================================================================
ISteamClient*					g_pSteamClient = NULL;
ISteamFriends*					g_pSteamFriends = NULL;
ISteamUtils*					g_pSteamUtils = NULL;
ISteamUser*						g_pSteamUser = NULL;
ISteamUserStats*				g_pSteamStats = NULL;
ISteamMatchmaking*				g_pMatchMaking = NULL;
ISteamMatchmakingServers*		g_pMatchMakingSvr =  NULL;
ISteamApps*						g_pSteamApps = NULL;
//===================================================================================
CreateInterfaceFn				g_SteamClientFactory = NULL;
//===================================================================================
DWORD WINAPI dwSteamThread( LPVOID lpArguments )
{
	HMODULE hmSteam = gBaseAPI.GetModuleHandleSafe( "SteamClient.dll" );
	g_SteamClientFactory = ( CreateInterfaceFn ) GetProcAddress( hmSteam, "CreateInterface" );

	if( !g_pSteamClient )
	{
		g_pSteamClient = ( ISteamClient* ) g_SteamClientFactory( STEAMCLIENT_INTERFACE_VERSION, NULL );
		XASSERT( g_pSteamClient );
	}

	HSteamPipe hNewPipe = g_pSteamClient->CreateSteamPipe( );
	HSteamUser hNewUser = g_pSteamClient->ConnectToGlobalUser( hNewPipe );

	if( !g_pSteamUtils )
	{
		g_pSteamUtils = g_pSteamClient->GetISteamUtils( hNewPipe, STEAMUTILS_INTERFACE_VERSION );
		XASSERT( g_pSteamUtils );
	}
	
	if( !g_pSteamFriends )
	{
		g_pSteamFriends	= g_pSteamClient->GetISteamFriends( hNewUser, hNewPipe, STEAMFRIENDS_INTERFACE_VERSION );
		XASSERT( g_pSteamFriends );
	}

	if( !g_pSteamUser )
	{
		g_pSteamUser = g_pSteamClient->GetISteamUser( hNewUser, hNewPipe, STEAMUSER_INTERFACE_VERSION );
		XASSERT( g_pSteamUser );
	}
	
	if( !g_pSteamStats )
	{
		g_pSteamStats = g_pSteamClient->GetISteamUserStats( hNewUser, hNewPipe, STEAMUSERSTATS_INTERFACE_VERSION );
		XASSERT( g_pSteamStats );
	}
	
	if( !g_pSteamApps )
	{
		g_pSteamApps = g_pSteamClient->GetISteamApps( hNewUser, hNewPipe, STEAMAPPS_INTERFACE_VERSION );
		XASSERT( g_pSteamApps );
	}

	if( !g_pMatchMaking )
	{
		g_pMatchMaking = g_pSteamClient->GetISteamMatchmaking( hNewUser, hNewPipe, STEAMMATCHMAKING_INTERFACE_VERSION );
		XASSERT( g_pMatchMaking );
	}

	if( !g_pMatchMakingSvr )
	{
		g_pMatchMakingSvr = g_pSteamClient->GetISteamMatchmakingServers( hNewUser, hNewPipe, STEAMMATCHMAKING_INTERFACE_VERSION );
		XASSERT( g_pMatchMakingSvr );
	}
#if DEBUG
	gBaseAPI.LogToFile( "g_pSteamClient: [0x%.8X]", (DWORD)g_pSteamClient );
	gBaseAPI.LogToFile( "g_pSteamUtils: [0x%.8X]", (DWORD)g_pSteamUtils );
	gBaseAPI.LogToFile( "g_pSteamFriends: [0x%.8X]", (DWORD)g_pSteamFriends );
	gBaseAPI.LogToFile( "g_pSteamUser: [0x%.8X]", (DWORD)g_pSteamUser );
	gBaseAPI.LogToFile( "g_pSteamStats: [0x%.8X]", (DWORD)g_pSteamStats );
	gBaseAPI.LogToFile( "g_pSteamApps: [0x%.8X]", (DWORD)g_pSteamApps );
	gBaseAPI.LogToFile( "g_pMatchMaking: [0x%.8X]", (DWORD)g_pMatchMaking );
	gBaseAPI.LogToFile( "g_pMatchMakingSvr: [0x%.8X]", (DWORD)g_pMatchMakingSvr );
#endif
	return 0;
}
//===================================================================================
DWORD WINAPI dwMainThread( LPVOID lpArguments )
{
	HMODULE hmClient = gBaseAPI.GetModuleHandleSafe( "client.dll" );
	g_ClientFactory = ( CreateInterfaceFn ) GetProcAddress( hmClient, "CreateInterface" );

	HMODULE hmEngine = gBaseAPI.GetModuleHandleSafe( "engine.dll" );
	g_EngineFactory = ( CreateInterfaceFn ) GetProcAddress( hmEngine, "CreateInterface" );

	HMODULE hmVGUI = gBaseAPI.GetModuleHandleSafe( "vguimatsurface.dll" );
	g_VGUIFactory = ( CreateInterfaceFn ) GetProcAddress( hmVGUI, "CreateInterface" );

	HMODULE hmVGUI2 = gBaseAPI.GetModuleHandleSafe( "vgui2.dll" );
	g_VGUI2Factory = ( CreateInterfaceFn ) GetProcAddress( hmVGUI2, "CreateInterface" );
		
	if( !g_pEngine )
	{
		g_pEngine = ( IVEngineClient* ) g_EngineFactory( VENGINE_CLIENT_INTERFACE_VERSION, NULL );
		XASSERT( g_pEngine );
	}

	if( !g_pClient )
	{
		g_pClient = ( IBaseClientDLL* ) g_ClientFactory( CLIENT_DLL_INTERFACE_VERSION, NULL );
		XASSERT( g_pClient );

		PDWORD* m_pdwClient = ( PDWORD* ) g_ClientFactory( CLIENT_DLL_INTERFACE_VERSION, NULL );

		if( m_pdwClient )
		{
			RtlCopyMemory( ( void* )&gClient,( void* )*m_pdwClient , sizeof ( CClient ) );
			
			PDWORD pdwAddress = ( PDWORD ) ( ( ( DWORD ) gClient.CreateMove ) + 0x28 );
			PDWORD pdwTable = ( PDWORD ) *pdwAddress;

			g_pClientHook = new CVMTHookManager( m_pdwClient );
			g_pClientHook->dwHookMethod( ( DWORD )Hooked_Init, 0 );
			//g_pClientHook->dwHookMethod( ( DWORD )Hooked_HudUpdate, 11 ); 
			g_pClientHook->dwHookMethod( ( DWORD )Hooked_IN_KeyEvent, 20 );
			g_pClientHook->dwHookMethod( ( DWORD )Hooked_CreateMove, 21 );

			if( !g_pEntList )
			{
				g_pEntList = ( IClientEntityList* ) g_ClientFactory( VCLIENTENTITYLIST_INTERFACE_VERSION, NULL );
				XASSERT( g_pEntList );
			}

			if( !g_pPrediction )
			{
				g_pPrediction = ( IPrediction* ) g_ClientFactory( VCLIENT_PREDICTION_INTERFACE_VERSION, NULL );
				XASSERT( g_pPrediction );
			}

			if( !g_pInput )
			{
				g_pInput = ( CInput* )*pdwTable;
				XASSERT( g_pInput );
			}
		}
	}

	if( !g_pEngineTrace )
	{
		g_pEngineTrace = ( IEngineTrace* ) g_EngineFactory( INTERFACEVERSION_ENGINETRACE_CLIENT, NULL );
		XASSERT( g_pEngineTrace );
	}

	if( !g_pEngineVGUI )
	{
		g_pEngineVGUI = ( IEngineVGui* ) g_EngineFactory( VENGINE_VGUI_VERSION, NULL );
		XASSERT( g_pEngineVGUI );
	}

	if( !g_pSurface )
	{
		g_pSurface = ( vgui::ISurface* ) g_VGUIFactory( VGUI_SURFACE_INTERFACE_VERSION, NULL );
		XASSERT( g_pSurface );
	}
	if( !g_pRandom )
	{
		g_pRandom = ( IUniformRandomStream* ) g_EngineFactory( VENGINE_CLIENT_RANDOM_INTERFACE_VERSION, NULL );
		XASSERT( g_pRandom );
	}

	if( !g_pIPanel )
	{
		g_pIPanel = ( IPanel* ) g_VGUI2Factory( VGUI_PANEL_INTERFACE_VERSION, NULL );
		XASSERT( g_pIPanel );

		if( g_pIPanel )
			BuildPanelHook( (PDWORD*)g_pIPanel );
	}
	if( !g_pModelInfo )
	{
		g_pModelInfo = ( IVModelInfoClient* ) g_EngineFactory( VMODELINFO_CLIENT_INTERFACE_VERSION, NULL );
		XASSERT( g_pModelInfo );
	}
	if (!g_pRenderView)
	{
		g_pRenderView = ( IVRenderView* ) g_EngineFactory(VENGINE_RENDERVIEW_INTERFACE_VERSION ,0);
		XASSERT( g_pRenderView );
	}
#if DEBUG
	gBaseAPI.LogToFile( "g_pEngine: [0x%.8X]", (DWORD)g_pEngine);
	gBaseAPI.LogToFile( "g_pClient: [0x%.8X]", (DWORD)g_pClient );
	gBaseAPI.LogToFile( "g_pEntList: [0x%.8X]", (DWORD)g_pEntList );
	gBaseAPI.LogToFile( "g_pPrediction: [0x%.8X]", (DWORD)g_pPrediction );
	gBaseAPI.LogToFile( "g_pInput: [0x%.8X]", (DWORD)g_pInput );
	gBaseAPI.LogToFile( "g_pEngineTrace: [0x%.8X]", (DWORD)g_pEngineTrace );
	gBaseAPI.LogToFile( "g_pEngineVGUI: [0x%.8X]", (DWORD)g_pEngineVGUI );
	gBaseAPI.LogToFile( "g_pSurface: [0x%.8X]", (DWORD)g_pSurface );
	gBaseAPI.LogToFile( "g_pRandom: [0x%.8X]", (DWORD)g_pRandom );
	gBaseAPI.LogToFile( "g_pIPanel: [0x%.8X]", (DWORD)g_pIPanel );
	gBaseAPI.LogToFile( "g_pModelInfo: [0x%.8X]", (DWORD)g_pModelInfo );
	gBaseAPI.LogToFile( "g_pRenderView: [0x%.8X]", (DWORD)g_pRenderView );
#endif
	return 0;
}

BOOL WINAPI DllMain( HINSTANCE hInstance, DWORD dwReasonOfCall, LPVOID lpReserved )
{
	if ( dwReasonOfCall == DLL_PROCESS_ATTACH )
	{
		
		hmThisModule = hInstance;
		gBaseAPI.BaseUponModule( (HMODULE)hInstance );
		gMemoryTools.RemovePEHeader( (DWORD)hInstance );
		//gMemoryTools.bHideModule((HMODULE)hInstance);
		DisableThreadLibraryCalls( (HMODULE)hInstance );

		CreateThread( NULL, 0, (LPTHREAD_START_ROUTINE)dwSteamThread, NULL, 0, NULL );
		CreateThread( NULL, 0, (LPTHREAD_START_ROUTINE)dwMainThread, NULL, 0, NULL );
		
	}

	return TRUE;
}