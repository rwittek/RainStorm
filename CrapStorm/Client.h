#ifndef _CLIENT_H_
#define _CLIENT_H_
//===================================================================================
#include <Windows.h>
#include "SDK.h"

//===================================================================================
// IBaseClientDLL interface from SDK
//===================================================================================
struct CClient
{
	// Called once when the client DLL is loaded
	int(__stdcall *Init)( CreateInterfaceFn appSystemFactory, CreateInterfaceFn physicsFactory,	CGlobalVarsBase *pGlobals );

	void (__stdcall *PostInit)( );

	// Called once when the client DLL is being unloaded
	void (__stdcall *Shutdown)( void );

	void (__stdcall *ReplayInit)( void );
	void (__stdcall *ReplayPostInit)( void );

	// Called at the start of each level change
	void (__stdcall *LevelInitPreEntity)( char const* pMapName );
	// Called at the start of a new level, after the entities have been received and created
	void (__stdcall *LevelInitPostEntity)( );
	// Called at the end of a level
	void (__stdcall *LevelShutdown)( void );

	// Request a pointer to the list of client datatable classes
	ClientClass *(__stdcall *GetAllClasses)( void ); //6

	// Called once per level to re-initialize any hud element drawing stuff
	int(__stdcall *HudVidInit)( void );
	// Called by the engine when gathering user input
	void (__stdcall *HudProcessInput)( bool bActive );
	// Called oncer per frame to allow the hud elements to think
	void (__stdcall *HudUpdate)( bool bActive ); //9
	// Reset the hud elements to their initial states
	void (__stdcall *HudReset)( void );
	// Display a hud text message
	void (__stdcall *HudText)( const char * message );

	// Mouse Input Interfaces
	// Activate the mouse (hides the cursor and locks it to the center of the screen)
	void (__stdcall *IN_ActivateMouse)( void );
	// Deactivates the mouse (shows the cursor and unlocks it)
	void (__stdcall *IN_DeactivateMouse)( void );
	// This is only called during extra sound updates and just accumulates mouse x, y offets and recenters the mouse.
	//  This call is used to try to prevent the mouse from appearing out of the side of a windowed version of the engine if 
	//  rendering or other processing is taking too long
	void (__stdcall *IN_Accumulate) (void);
	// Reset all key and mouse states to their initial, unpressed state
	void (__stdcall *IN_ClearStates )(void);
	// If key is found by name, returns whether it's being held down in isdown, otherwise function returns false
	bool (__stdcall *IN_IsKeyDown)( const char *name, bool& isdown );

	int (__stdcall *IN_OnMouseWheeled)(int);

	// Raw keyboard signal, if the client .dll returns 1, the engine processes the key as usual, otherwise,
	//  if the client .dll returns 0, the key is swallowed.
	int(__stdcall *IN_KeyEvent)( int eventcode, int keynum, const char *pszCurrentBinding ); //17

	// This function is called once per tick to create the player CUserCmd (used for prediction/physics simulation of the player)
	// Because the mouse can be sampled at greater than the tick interval, there is a separate input_sample_frametime, which
	//  specifies how much additional mouse / keyboard simulation to perform.
	void (__stdcall *CreateMove )( 
		int sequence_number,			// sequence_number of this cmd
		float input_sample_frametime,	// Frametime for mouse input sampling
		bool active );// True if the player is active (not paused) //18

	// If the game is running faster than the tick_interval framerate, then we do extra mouse sampling to avoid jittery input
	//  This code path is much like the normal move creation code, except no move is created
	void (__stdcall *ExtraMouseSample)( float frametime, bool active );

	// Encode the delta (changes) between the CUserCmd in slot from vs the one in slot to.  The game code will have
	//  matching logic to read the delta.
	bool (__stdcall *WriteUsercmdDeltaToBuffer)( bf_write *buf, int from, int to, bool isnewcommand ); //20
	// Demos need to be able to encode/decode CUserCmds to memory buffers, so these functions wrap that
	void (__stdcall *EncodeUserCmdToBuffer)( bf_write& buf, int slot ); //21
	void (__stdcall *DecodeUserCmdFromBuffer)( bf_read& buf, int slot ); //22

	// Set up and render one or more views (e.g., rear view window, etc.).  This called into RenderView below
	void (__stdcall *View_Render)( vrect_t *rect );

	// Allow engine to expressly render a view (e.g., during timerefresh)
	// See IVRenderView.h, PushViewFlags_t for nFlags values
	void (__stdcall *RenderView)( const CViewSetup &view, int nClearFlags, int whatToDraw );

	// Apply screen fade directly from engine
	void (__stdcall *View_Fade)( ScreenFade_t *pSF );

	// The engine has parsed a crosshair angle message, this function is called to dispatch the new crosshair angle
	void (__stdcall *SetCrosshairAngle)( const QAngle& angle ); //26 

	// Sprite (.spr) model handling code
	// Load a .spr file by name
	void (__stdcall *InitSprite)( CEngineSprite *pSprite, const char *loadname );
	// Shutdown a .spr file
	void (__stdcall *ShutdownSprite)( CEngineSprite *pSprite );
	// Returns sizeof)( CEngineSprite ) so the engine can allocate appropriate memory
	int(__stdcall *GetSpriteSize)( void );

	// Called when a player starts or stops talking.
	// entindex is -1 to represent the local client talking (before the data comes back from the server). 
	// entindex is -2 to represent the local client's voice being acked by the server.
	// entindex is GetPlayer() when the server acknowledges that the local client is talking.
	void (__stdcall *VoiceStatus)( int entindex, qboolean bTalking );

	// Networked string table definitions have arrived, allow client .dll to 
	//  hook string changes with a callback function )( see INetworkStringTableClient.h )
	void (__stdcall *InstallStringTableCallback)( char const *tableName );

	// Notification that we're moving into another stage during the frame.
	void (__stdcall *FrameStageNotify)( ClientFrameStage_t curStage ); //32

	// The engine has received the specified user message, this code is used to dispatch the message handler
	bool (__stdcall *DispatchUserMessage)( int msg_type, bf_read &msg_data ); //33

	// Save/restore system hooks
	CSaveRestoreData  *(__stdcall *SaveInit)( int size );
	void (__stdcall *SaveWriteFields)( CSaveRestoreData *, const char *, void *, datamap_t *, typedescription_t *, int );
	void (__stdcall *SaveReadFields)( CSaveRestoreData *, const char *, void *, datamap_t *, typedescription_t *, int );
	void (__stdcall *PreSave)( CSaveRestoreData * );
	void (__stdcall *Save)( CSaveRestoreData * );
	void (__stdcall *WriteSaveHeaders)( CSaveRestoreData * );
	void (__stdcall *ReadRestoreHeaders)( CSaveRestoreData * );
	void (__stdcall *Restore)( CSaveRestoreData *, bool );
	void (__stdcall *DispatchOnRestore)();

	// Hand over the StandardRecvProxies in the client DLL's module.
	CStandardRecvProxies *(__stdcall *GetStandardRecvProxies)();

	// save game screenshot writing
	void (__stdcall *WriteSaveGameScreenshot)( const char *pFilename );

	// Given a list of "S(wavname) S(wavname2)" tokens, look up the localized text and emit
	//  the appropriate close caption if running with closecaption = 1
	void (__stdcall *EmitSentenceCloseCaption)( char const *tokenstream );
	// Emits a regular close caption by token name
	void (__stdcall *EmitCloseCaption)( char const *captionname, float duration );

	// Returns true if the client can start recording a demo now.  If the client returns false,
	// an error message of up to length bytes should be returned in errorMsg.
	bool (__stdcall *CanRecordDemo)( char *errorMsg, int length );

	// Added interface

	// save game screenshot writing
	void (__stdcall *WriteSaveGameScreenshotOfSize)( const char *pFilename, int width, int height );

	// Gets the current view
	bool (__stdcall *GetPlayerView)( CViewSetup &playerView );

	// Matchmaking
	void (__stdcall *SetupGameProperties)( CUtlVector< XUSER_CONTEXT > &contexts, CUtlVector< XUSER_PROPERTY > &properties );
	uint (__stdcall *GetPresenceID)( const char *pIDName );
	const char *(__stdcall *GetPropertyIdString)( const uint id );
	void (__stdcall *GetPropertyDisplayString)( uint id, uint value, char *pOutput, int nBytes );
#ifdef _WIN32
	void (__stdcall *StartStatsReporting)( HANDLE handle, bool bArbitrated );
#endif

	void (__stdcall *InvalidateMdlCache)();

	void (__stdcall *IN_SetSampleTime)( float frametime );
};
//===================================================================================
extern CClient gClient;
//===================================================================================
extern int  __stdcall Hooked_Init( CreateInterfaceFn AppSysFactory, CreateInterfaceFn PhysicsFactory, CGlobalVarsBase* g_pGlobals );
extern void __stdcall Hooked_HudUpdate( bool bActive );
extern void __stdcall Hooked_CreateMove( int sequence_number, float input_sample_frametime, bool active );
extern int  __stdcall Hooked_IN_KeyEvent ( int eventcode, int keynum, const char *pszCurrentBinding );
//===================================================================================
extern void (*SetPredictionRandomSeed)(const CUserCmd *cmd); 

struct COffsets
{
public:
	int m_bReadyToBackstab;
	int m_hActiveWeapon;
	int m_lifeState;
	int m_iHealth;
	int m_iHealth_object;
	int m_iPlayerClass;
	int m_iClass;
	int m_iTeamNum;
	int m_bHasSapper;
    int m_iUpgradeLevel;
	int m_iUpgradeMetal;
	int m_Shared;
	int m_nPlayerCond;
	float m_flPercentageConstructed;
	int m_bBuilding;
	int m_iAmmoShells;
	int m_iState;
	int m_bPlayerControlled;
	int m_iAmmoRockets;
	int m_iAmmoMetal;
};
extern COffsets gOffsets;

class CVerifiedUserCmd
{
public:
	CUserCmd	m_cmd;
	CRC32_t		m_crc;
};

#endif