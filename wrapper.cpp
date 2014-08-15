#include <windows.h>
#include <tlhelp32.h>
#include <winsock.h>
#include <algorithm>
#include <time.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/timeb.h>
#include <vector>
#include <fstream>
#include <istream>
#include <string.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <fcntl.h>

#define _USRDLL
#define TF2BASE_EXPORTS
#define TF_CLIENT_DLL
#define CLIENT_DLL
#define private public
#define protected public
#include "public\cdll_int.h"
#include "public\iprediction.h"
#include "public\client\imessagechars.h"
#include "public\client\iclientmode.h"
#include "public\client\cliententitylist.h"
#include "public\client\cdll_client_int.h"
#include "public\client\cbase.h"
#include "c_sdk_player.h"
#include "public\client\c_baseanimating.h"
#include "public\client\ivieweffects.h"
#include "public\client\c_basecombatweapon.h"
#include "public\client\c_baseplayer.h"
#include "public\client\c_baseentity.h"
#include "public\icliententitylist.h"
#include "public\engine/ivmodelrender.h"
#include "public\iefx.h"
#include "public\icvar.h"
#include "public\ivrenderview.h"
#include "public\engine/ivdebugoverlay.h"
#include "public\materialsystem/imaterialsystemstub.h"
#include "public\engine\ivmodelinfo.h"
#include "public\ienginevgui.h"
#include "public\networkstringtabledefs.h"
#include "public\ispatialpartition.h"
#include "public\engine\ishadowmgr.h"
#include "public\engine\IStaticPropMgr.h"
#include "public\engine\IEngineSound.h"
#include "public\vstdlib/random.h"
#include "public\VGuiMatSurface/IMatSystemSurface.h"
#include "public\vgui\Cursor.h"
#include "public\vgui\Dar.h"
#include "public\vgui\IBorder.h"
#include "public\vgui\IClientPanel.h"
#include "public\vgui\IPanel.h"
#include "vgui_controls/controls.h"
#include "vgui\ISurface.h"
#include "vgui_controls\Panel.h"
#include "public\engine\IEngineTrace.h"
#include "public\IGameUIFuncs.h"
#include "public\igameevents.h"
#include "public\client\input.h"
#include "public\shared\usermessages.h"
#include "public\vgui\IInputInternal.h"
#include "public\vgui_controls\Frame.h"
#include "public\vgui_controls\CheckButton.h"
#include "public\vgui_controls\ComboBox.h"
#include "public\vgui_controls\Button.h"
#include "public\vgui_controls\Controls.h"
#include "public\vgui_controls\DialogManager.h"
#include "public\vgui_controls\RadioButton.h"
#include "public\vgui_controls\Menu.h"
#include "public\client/game_controls/commandmenu.h"
#include "public\tier1\convar.h"
#include "public\shared\basecombatweapon_shared.h"
#include "public\shared\takedamageinfo.h"
#include "public\vphysics_interface.h"
#include "public\shake.h"
#include "public\dlight.h"
#include "public\iefx.h" 
#include "public\igameevents.h"
#include "public\materialsystem\IMaterialVar.h"
#include "public\vgui\ILocalize.h"
#include "public\engine\ivdebugoverlay.h"
#include "public\igameresources.h"
#include "public\inetchannelinfo.h"
#include "public\inputsystem\iinputsystem.h"
#include "public\iachievementmgr.h"
#include "public\shared\achievementmgr.h"
#include "public\steam\steam_api.h"
#include "public\IGameUIFuncs.h"
#include "public\toolframework\IEngineTool.h"
#include "public\inetchannel.h"
#include "public\inetmessage.h"
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
//========= Copyright Valve Corporation, All rights reserved. ============//
//
// Purpose: Generic CRC functions
//
//=============================================================================//

#include "basetypes.h"
#include "commonmacros.h"
#include "checksum_crc.h"

// memdbgon must be the last include file in a .cpp file!!!
#include "tier0/memdbgon.h"

#define CRC32_INIT_VALUE 0xFFFFFFFFUL
#define CRC32_XOR_VALUE  0xFFFFFFFFUL

#define NUM_BYTES 256
static const CRC32_t pulCRCTable[NUM_BYTES] =
{
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba,
    0x076dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
    0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988,
    0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
    0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de,
    0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
    0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec,
    0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5,
    0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172,
    0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940,
    0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
    0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116,
    0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
    0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924,
    0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d,
    0x76dc4190, 0x01db7106, 0x98d220bc, 0xefd5102a,
    0x71b18589, 0x06b6b51f, 0x9fbfe4a5, 0xe8b8d433,
    0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818,
    0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e,
    0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457,
    0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea, 0xfcb9887c,
    0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
    0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2,
    0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb,
    0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0,
    0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
    0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086,
    0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4,
    0x59b33d17, 0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad,
    0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a,
    0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683,
    0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8,
    0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1,
    0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe,
    0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7,
    0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc,
    0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252,
    0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
    0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60,
    0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4669be79,
    0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236,
    0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f,
    0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04,
    0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
    0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a,
    0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38,
    0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21,
    0x86d3d2d4, 0xf1d4e242, 0x68ddb3f8, 0x1fda836e,
    0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
    0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c,
    0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
    0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2,
    0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db,
    0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0,
    0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6,
    0xbad03605, 0xcdd70693, 0x54de5729, 0x23d967bf,
    0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94,
    0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d
};

void CRC32_Init(CRC32_t *pulCRC)
{
	*pulCRC = CRC32_INIT_VALUE;
}

void CRC32_Final(CRC32_t *pulCRC)
{
	*pulCRC ^= CRC32_XOR_VALUE;
}

CRC32_t	CRC32_GetTableEntry( unsigned int slot )
{
	return pulCRCTable[(unsigned char)slot];
}

void CRC32_ProcessBuffer(CRC32_t *pulCRC, const void *pBuffer, int nBuffer)
{
	CRC32_t ulCrc = *pulCRC;
	unsigned char *pb = (unsigned char *)pBuffer;
    unsigned int nFront;
    int nMain;

JustAfew:

    switch (nBuffer)
    {
    case 7:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 6:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 5:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 4:
        ulCrc ^= LittleLong( *(CRC32_t *)pb );
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
		*pulCRC = ulCrc;
        return;

    case 3:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 2:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 1:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);

    case 0:
		*pulCRC = ulCrc;
        return;
    }

    // We may need to do some alignment work up front, and at the end, so that
    // the main loop is aligned and only has to worry about 8 byte at a time.
    //
    // The low-order two bits of pb and nBuffer in total control the
    // upfront work.
    //
    nFront = ((unsigned int)pb) & 3;
    nBuffer -= nFront;
    switch (nFront)
    {
    case 3:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);
    case 2:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);
    case 1:
        ulCrc  = pulCRCTable[*pb++ ^ (unsigned char)ulCrc] ^ (ulCrc >> 8);
    }

    nMain = nBuffer >> 3;
    while (nMain--)
    {
        ulCrc ^= LittleLong( *(CRC32_t *)pb );
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc ^= LittleLong( *(CRC32_t *)(pb + 4) );
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        ulCrc  = pulCRCTable[(unsigned char)ulCrc] ^ (ulCrc >> 8);
        pb += 8;
    }

    nBuffer &= 7;
    goto JustAfew;
}

FILE *logfile;

CreateInterfaceFn AppSysFactory;
extern "C" void rainstorm_command_cb(const char *arguments);
void rainstorm_command_cb_trampoline( const CCommand &args ) { rainstorm_command_cb(args.ArgS()); }

ConCommand rainstorm_command("rainstorm", rainstorm_command_cb_trampoline);

bool	bDataCompare( const BYTE* pData, const BYTE* bMask, const char* szMask )
{
	for( ; *szMask; ++szMask, ++pData, ++bMask )
		if( *szMask == 'x' && *pData != *bMask )
			return false;

	return ( *szMask ) == NULL;
}
DWORD dwFindPattern ( DWORD dwAddress, DWORD dwSize, BYTE* pbMask, char* szMask )
{
	for( DWORD i = NULL; i < dwSize; i++ )
		if( bDataCompare( (BYTE*) ( dwAddress + i ), pbMask, szMask ) )
			return (DWORD)( dwAddress + i );

	return 0;
}

extern "C" CInput *CINPUT_PTR;
extern "C" bool NOCMD_ENABLED;
IClientEntityList *ENTLISTPTR;
extern "C" int ( __stdcall *REAL_INIT)( CreateInterfaceFn appSysFactory, CreateInterfaceFn physicsFactory, CGlobalVarsBase* pGlobals );
extern "C" void (__stdcall *REAL_CREATEMOVE)( int sequence_number, float input_sample_frametime, bool active );
extern "C" void (__stdcall *REAL_EXTRAMOUSESAMPLE)( float input_sample_frametime, bool active );
extern "C" void (__fastcall *REAL_SERVERCMDKEYVALUES)( IVEngineClient *_this, int edx, KeyValues *kv );
extern "C" bool (__fastcall *REAL_NETCHANNEL_SENDDATAGRAM)(INetChannel *chan, int ignoreme, bf_write *data);
bool (__fastcall *COPIED_NETCHANNEL_SENDDATAGRAM)(INetChannel *chan, int ignoreme, bf_write *data) = NULL;
extern "C" void rainstorm_preinithook( CreateInterfaceFn appSysFactory, CreateInterfaceFn physicsFactory, CGlobalVarsBase* pGlobals );
extern "C" void rainstorm_postinithook();
extern "C" void rainstorm_pre_createmove(int *sequence_number_ptr, float *input_sample_frametime_ptr, bool *active_ptr);
extern "C" void rainstorm_process_usercmd(CUserCmd *cmd);
extern "C" void rainstorm_extramousesample(float input_sample_frametime, bool active);
extern "C" IVEngineClient *rainstorm_getivengineclient();
extern "C" void rainstorm_init(int log_fd, void * hooked_init_trampoline, void *hooked_createmove_trampoline, void *hooked_extramousesample_trampoline);
extern "C" int LOG_FD;
CGlobalVarsBase *globals_ptr = NULL;
void __stdcall hooked_servercmdkeyvalues( KeyValues *kv ) {
	fprintf(logfile, "KeyValues handling...\n");
	//KeyValuesDumpAsDevMsg(kv);
	//REAL_SERVERCMDKEYVALUES(_this, edx, kv);
}
extern "C" void *get_hooked_servercmdkeyvalues() {
	return &hooked_servercmdkeyvalues;
}
int __stdcall hooked_init_trampoline( CreateInterfaceFn appSysFactory, CreateInterfaceFn physicsFactory, CGlobalVarsBase* pGlobals ) {
	globals_ptr = pGlobals;
	AppSysFactory = appSysFactory;
	if (REAL_INIT != NULL) {
		rainstorm_preinithook(appSysFactory, physicsFactory, pGlobals);
		int retval = (*REAL_INIT)(appSysFactory, physicsFactory, pGlobals);
		rainstorm_postinithook();
		return retval;
	} else {
		//MessageBox(NULL, "no init :(", NULL, NULL);
		while (1) {;};
	}
}

void __stdcall hooked_createmove_trampoline( int sequence_number, float input_sample_frametime, bool active )
{
	rainstorm_pre_createmove(&sequence_number, &input_sample_frametime, &active);
	
	(*REAL_CREATEMOVE)( sequence_number, input_sample_frametime, active );
	IVEngineClient *engine = rainstorm_getivengineclient();
	
	if( engine->IsLevelMainMenuBackground( ) || engine->IsDrawingLoadingImage( ) || engine->IsInGame( ) == false )
		return;
		
		
	static CUserCmd* pCommands = *(CUserCmd**)((DWORD)CINPUT_PTR + 0xC4);  
	CUserCmd* pCommand = &pCommands[ sequence_number % 90 ];

	rainstorm_process_usercmd(pCommand);
	
	CVerifiedUserCmd *pSafeCommand = *reinterpret_cast<CVerifiedUserCmd**>((size_t)CINPUT_PTR + 0xC8) + (sequence_number%90);
	pSafeCommand->m_cmd = *pCommand;
	pSafeCommand->m_crc = pSafeCommand->m_cmd.GetChecksum();
}
void __stdcall hooked_extramousesample_trampoline( float input_sample_frametime, bool active )
{
	rainstorm_extramousesample(input_sample_frametime, active);
	REAL_EXTRAMOUSESAMPLE(input_sample_frametime, active);
}

extern "C" Vector c_baseentity_getvelocity(C_BaseEntity *ent) {
	return ent->GetBaseVelocity() + ent->GetLocalVelocity();
}

bool __fastcall hooked_netchannel_senddatagram_trampoline(INetChannel *chan, int ignoreme, bf_write *data) {
	//fprintf(logfile, "SendDatagram\n");
	/*if (COPIED_NETCHANNEL_SENDDATAGRAM == NULL) {
		fprintf(logfile, "Copying!\n");
		unsigned long oldprotect;
		VirtualProtect((void *)((int)REAL_NETCHANNEL_SENDDATAGRAM & 0xFFFFF000), 6000, PAGE_READWRITE, &oldprotect);
		*(unsigned short *)((int)REAL_NETCHANNEL_SENDDATAGRAM + 0x198) = 0x9090; // nop it out
		VirtualProtect((void *)((int)REAL_NETCHANNEL_SENDDATAGRAM & 0xFFFFF000), 6000, PAGE_EXECUTE, &oldprotect);
		FlushInstructionCache(GetCurrentProcess(), NULL, 0);
		COPIED_NETCHANNEL_SENDDATAGRAM = REAL_NETCHANNEL_SENDDATAGRAM;
	}*/
	
	if (NOCMD_ENABLED) {
		return true;
	} else {
		return REAL_NETCHANNEL_SENDDATAGRAM(chan, ignoreme, data);
	}
}

CUserCmd* __stdcall Hooked_GetUserCmd( int sequence_number ) 
{ 
    static CUserCmd* pCommands = *(CUserCmd**)((DWORD)CINPUT_PTR + 0xC4); 
    return &pCommands[ sequence_number % 90 ]; 
}
extern "C" void *get_hooked_getusercmd() {
	return &Hooked_GetUserCmd;
}

extern "C" void *get_netchannel_senddatagram_trampoline() {
	return &hooked_netchannel_senddatagram_trampoline;
}


extern "C" INetChannel *get_current_inetchannel(IVEngineClient *engine) {
	return (INetChannel *) engine->GetNetChannelInfo();
}
extern "C" ICvar * getptr_icvar(CreateInterfaceFn unused) {
	ICvar *ptr = (ICvar *)AppSysFactory( CVAR_INTERFACE_VERSION, NULL );
	// ewwwww.
	ptr->RegisterConCommand((ConCommandBase *)&rainstorm_command);
	return ptr;
}
extern "C" float get_current_latency(IVEngineClient *engine) {
	float netlag = get_current_inetchannel(engine)->GetLatency(0);
	return netlag;
}

DWORD WINAPI startup_thread( LPVOID lpArguments ) {
	logfile = fopen("rainstorm_debug.txt", "w");
	setbuf(logfile, NULL);
	
	rainstorm_init(fileno(logfile), (void*)&hooked_init_trampoline, (void*)&hooked_createmove_trampoline, (void *)&hooked_extramousesample_trampoline);
	//exit(1);
	return 0;
}

extern "C" BOOL APIENTRY DllMain( HINSTANCE hInstance, DWORD dwReasonOfCall, LPVOID lpReserved ) {
	if ( dwReasonOfCall == DLL_PROCESS_ATTACH ) {
		CreateThread( NULL, 0, (LPTHREAD_START_ROUTINE)startup_thread, NULL, 0, NULL );
	}
	return true;
}

HMODULE GetModuleHandleSafe( const char* pszModuleName )
{
	HMODULE hmModuleHandle = NULL;

	do
	{
		hmModuleHandle = GetModuleHandle( pszModuleName );
		Sleep( 1 );
	}
	while(hmModuleHandle == NULL);

	return hmModuleHandle;
}

CreateInterfaceFn ClientFactory;
CreateInterfaceFn EngineFactory;
void setup_clientfactory() {
	HMODULE hmClient = GetModuleHandleSafe( "client.dll" );
	ClientFactory = ( CreateInterfaceFn ) GetProcAddress( hmClient, "CreateInterface" );
}
void setup_enginefactory() {
	HMODULE hmClient = GetModuleHandleSafe( "engine.dll" );
	ClientFactory = ( CreateInterfaceFn ) GetProcAddress( hmClient, "CreateInterface" );
}
extern "C" IBaseClientDLL * getptr_ibaseclientdll() {
	HMODULE hmClient = GetModuleHandleSafe( "client.dll" );
	if (ClientFactory == NULL) setup_clientfactory();
	return (IBaseClientDLL *)ClientFactory ( CLIENT_DLL_INTERFACE_VERSION, NULL );
}

extern "C" IClientEntityList * getptr_icliententitylist () {
	if (ClientFactory == NULL) setup_clientfactory();
	return (IClientEntityList*) ClientFactory ( VCLIENTENTITYLIST_INTERFACE_VERSION, NULL );
}
CUniformRandomStream GlobalStream;

extern "C" IUniformRandomStream * getptr_iuniformrandomstream () {
	return &GlobalStream;
}


// HACK HACK REMOVE THIS!!!!
extern "C" bool is_shoot_critical(int iSeed, C_BaseCombatWeapon *pBaseWeapon) {
	return false;
	//sig from crapstorm
	//dwFindPattern( (DWORD)GetModuleHandle("client.dll"), 0x5F6000, (BYTE*)"\x8B\x44\x24\x04\x85\xC0\x75\x0B\xC7\x05\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xC3", "xxxxxxxxxx????xxxxx" );
	//(int *)((DWORD)GetModuleHandleA( "client.dll" ) + 0xB1AFE8); 
             
    // typedef bool ( __thiscall* CalcIsAttackCriticalHelper)( C_BaseCombatWeapon* ); 

	// int *wepseed = (int *)(((int)pBaseWeapon)+0x0AB4 + 8);
	// int oldwepseed = *wepseed;
	// int oldglobalseed = *pGlobalSeed;
    // *pGlobalSeed = iSeed;
	
    // /*int weaponSeedBackup  = *Ptr(int*, pBaseWeapon, 0xA4C);  
    // float checkTimeBackup = *Ptr(float*, pBaseWeapon, 0xA44);  
    // float critTimeBackup  = *Ptr(float*, pBaseWeapon, 0xA40);  
      // */   
	// fprintf(logfile, "%p\n", ((PDWORD*)pBaseWeapon)[ 388 ]);
	
    // bool bResult = ( (CalcIsAttackCriticalHelper)(*(PDWORD*)pBaseWeapon)[ 388 ] )( pBaseWeapon ); 
     
	 // *wepseed = oldwepseed;
	 // *pGlobalSeed = oldglobalseed;
    // /* *Ptr(int*, pBaseWeapon, 0xA4C)    	= weaponSeedBackup; 
    // *Ptr(float*, pBaseWeapon, 0xA44) = checkTimeBackup; 
    // *Ptr(float*, pBaseWeapon, 0xA40) = critTimeBackup; */
	
    // return bResult; 
}
extern "C" IVEngineClient * getptr_ivengineclient() {
	HMODULE hmEngine = GetModuleHandleSafe( "engine.dll" );
	CreateInterfaceFn EngineFactory = ( CreateInterfaceFn ) GetProcAddress( hmEngine, "CreateInterface" );
	return (IVEngineClient *)EngineFactory( VENGINE_CLIENT_INTERFACE_VERSION, NULL );
}

extern "C" IEngineTrace * getptr_ienginetrace() {
	HMODULE hmEngine = GetModuleHandleSafe( "engine.dll" );
	CreateInterfaceFn EngineFactory = ( CreateInterfaceFn ) GetProcAddress( hmEngine, "CreateInterface" );
	return ( IEngineTrace* ) EngineFactory( INTERFACEVERSION_ENGINETRACE_CLIENT, NULL );
}
extern "C" IVModelInfo * getptr_ivmodelinfo() {
	HMODULE hmEngine = GetModuleHandleSafe( "engine.dll" );
	CreateInterfaceFn EngineFactory = ( CreateInterfaceFn ) GetProcAddress( hmEngine, "CreateInterface" );
	return ( IVModelInfo * ) EngineFactory( VMODELINFO_CLIENT_INTERFACE_VERSION, NULL );
}



extern "C" IEngineTool * getptr_ienginetool() {
	return (IEngineTool *) AppSysFactory( VENGINETOOL_INTERFACE_VERSION, NULL );
}
extern "C" void iuniformrandomstream_set_seed(IUniformRandomStream *stream, int seed) {
	stream->SetSeed(seed);
}
extern "C" int iuniformrandomstream_random_int(IUniformRandomStream *stream, int minval, int maxval) {
	return stream->RandomInt(minval, maxval);
}
extern "C" void * icvar_findvar(ICvar *icvar, const char *name) {
	return icvar->FindVar(name);
}

extern "C" void convar_setvalue_raw_int(ConVar *cvar, int value) {
	cvar->m_nValue = value; // bypasses SMAC, etc.
}
extern "C" void convar_setvalue_str(ConVar *cvar, const char *value) {
	cvar->SetValue(value);
}
extern "C" float convar_getvalue_float(ConVar *cvar) {
	return cvar->GetFloat();
}
extern "C" void convar_setvalue_float(ConVar *cvar, float val) {
	cvar->SetValue(val);
}
extern "C" void concommand_clearflags(ConVar *pVar) {
	pVar->m_nFlags = FCVAR_NONE;
}
extern "C" void _Unwind_Resume() {
	while (1) {;};
}
extern "C" int trace_t_gethitgroup(CGameTrace *tr) {
	return tr->hitgroup;
}

void convar_restore( IConVar* ivar, const char* pOldValue, float flOldValue ) {
	ConVar *var = (ConVar *) ivar;
	if (pOldValue && strlen(pOldValue)) {
		var->m_pszString = (char *) pOldValue;
		var->m_StringLength = strlen(pOldValue);
	}
	//else var->InternalSetFloatValue(flOldValue);
}

extern "C" void convar_changeandfreeze(ConVar *cvar, const char *newval) {
	//auto oldcb = cvar->m_fnChangeCallback;
	cvar->m_fnChangeCallback = NULL;
	cvar->InternalSetValue(newval);
	cvar->m_fnChangeCallback = NULL;
}


extern "C" void ivengineclient_clientcmd(IVEngineClient *engine_ptr, const char *command) {
	engine_ptr->ClientCmd(command);
}

extern "C" float ivengineclient_time(IVEngineClient *engine_ptr)  {
	return engine_ptr->Time();
}
extern "C" int ivengineclient_getlocalplayer(IVEngineClient *engine_ptr) {
	return engine_ptr->GetLocalPlayer();
}
extern "C" ClientClass *ibaseclientdll_getallclasses(IBaseClientDLL *client) {
	return client->GetAllClasses();
}
extern "C" void angle_vectors(QAngle &angle, Vector *vec1, Vector *vec2, Vector *vec3) {
	AngleVectors(angle, vec1, vec2, vec3);
}
extern "C" void vector_angles(Vector *vec, QAngle *angles) {
	VectorAngles(*vec, *angles);
}
extern "C" float vector_length(Vector *vec) {
	return vec->Length();
}
extern "C" void ray_t_init(Ray_t &ray, Vector &start, Vector &end) {
	ray.Init(start, end);
}
extern "C" IClientEntity *icliententitylist_getcliententity(IClientEntityList *client_entity_list, int ent_index) {
	return client_entity_list->GetClientEntity(ent_index);
}
extern "C" IClientEntity *icliententitylist_getcliententityfromhandle(IClientEntityList *client_entity_list, CBaseHandle handle) {
	return client_entity_list->GetClientEntityFromHandle(handle);
}

extern "C" Vector c_baseentity_getorigin(C_BaseEntity *ent) {
	return ent->GetAbsOrigin();
}
extern "C" Vector c_baseentity_worldspacecenter(C_BaseEntity *ent) {
	return ent->WorldSpaceCenter();
}
extern "C" bool c_baseentity_isalive(C_BaseEntity *ent) {
	return ent->IsAlive();
}
extern "C" int c_baseentity_getindex(C_BaseEntity *ent) {
	return ent->index;
}
void get_bone_position(C_BaseAnimating *ent, IVModelInfo *modelinfo, int iBone, Vector &origin)
{
	studiohdr_t *pStudioHdr = modelinfo->GetStudiomodel( ent->GetModel( ) );

	if (!pStudioHdr)
		return;

	if (iBone < 0 || iBone >= pStudioHdr->numbones)
		return;

	matrix3x4_t bonetoworld[128];

	if( !ent->SetupBones( bonetoworld, MAXSTUDIOBONES, BONE_USED_BY_HITBOX, 0) )
        return;

	float x = bonetoworld[iBone][0][3];
	float y = bonetoworld[iBone][1][3];
	float z = bonetoworld[iBone][2][3];

	origin = Vector(x, y, z);
}
void get_hitbox_position(C_BaseAnimating *ent, IVModelInfo *modelinfo, int hitboxIndex, Vector &origin )
{

	int bone = -1;
	studiohdr_t *pStudioHdr = modelinfo->GetStudiomodel(ent->GetModel());
	Vector bbmax, bbmin;
	if ( pStudioHdr )
	{
		mstudiohitboxset_t *set =pStudioHdr->pHitboxSet( ent->m_nHitboxSet );
		if ( set && hitboxIndex < set->numhitboxes )
		{
			auto hitbox = set->pHitbox( hitboxIndex );
			bbmax = hitbox->bbmax;
			bbmin = hitbox->bbmin;
			bone = hitbox->bone;
		}
	}
	if (bone == -1) return;
	
	matrix3x4_t bonetoworld[128];

	if( !ent->SetupBones( bonetoworld, MAXSTUDIOBONES, BONE_USED_BY_HITBOX, 0) )
        return;

	Vector bbmax_world;
	Vector bbmin_world;
	VectorTransform(bbmax, bonetoworld[bone], bbmax_world);
	VectorTransform(bbmin, bonetoworld[bone], bbmin_world);
	
	origin = (bbmax_world + bbmin_world)/2.0;
	globals_ptr->curtime -= 1.0/66.0;
}

extern "C" void c_baseanimating_gethitboxposition(C_BaseAnimating *ent, IVModelInfo *modelinfo, int iHitbox, Vector &origin ) {
	get_hitbox_position(ent, modelinfo, iHitbox, origin);
}
extern "C" void c_baseanimating_getboneposition(C_BaseAnimating *ent, IVModelInfo *modelinfo, int iBone, Vector &origin) {
	get_bone_position(ent, modelinfo, iBone, origin);
}
extern "C" int c_baseanimating_getnumbones(C_BaseAnimating *ent, IVModelInfo *modelinfo) {
	studiohdr_t *pStudioHdr = modelinfo->GetStudiomodel( ent->GetModel( ) );
	if (pStudioHdr) {
		return pStudioHdr->numbones;
	} else {
		return 0;
	}
}
extern "C" int c_baseanimating_getnumhitboxes(C_BaseAnimating *ent, IVModelInfo *modelinfo) {
	studiohdr_t *pStudioHdr = modelinfo->GetStudiomodel( ent->GetModel( ) );
	if (pStudioHdr) {
		mstudiohitboxset_t *set =pStudioHdr->pHitboxSet( ent->m_nHitboxSet );
		if ( set  )
		{
			return set-> numhitboxes;
		} else {
			return 0;
		}
	} else {
		return 0;
	}
}
extern "C" void ivengineclient_setviewangles(IVEngineClient *eng, QAngle &ang) {
	eng->SetViewAngles(ang);
}
extern "C" void ibaseclientdll_setcrosshairangles(IBaseClientDLL *client, QAngle &ang) {
	client->SetCrosshairAngle(ang);
}
extern "C" size_t ivengineclient_getplayername(IVEngineClient *eng, C_BaseEntity *ent, char *buf, size_t bufsize) {
	player_info_t info;
	if (eng->GetPlayerInfo(ent->index, &info)) {
		strncpy(buf, info.name, strlen(info.name));
		return strlen(info.name);
	}
	//buf[0] = 0;
	return 0;
}
extern "C" void ienginetrace_traceray(IEngineTrace *enginetrace, const Ray_t &ray, unsigned int fMask, ITraceFilter *pTraceFilter, trace_t *pTrace ) {
	enginetrace->TraceRay(ray, fMask, pTraceFilter, pTrace);
}
class TriggerbotTraceFilter : public ITraceFilter
{
    virtual bool ShouldHitEntity( IHandleEntity *pEntity, int contentsMask );
    virtual TraceType_t  GetTraceType() const;
};
bool TriggerbotTraceFilter::ShouldHitEntity( IHandleEntity* pHandle, int contentsMask )
{
    CBaseEntity* pEnt = static_cast<CBaseEntity*>( pHandle );
	CBaseEntity *me = static_cast<C_BaseEntity*>(getptr_icliententitylist()->GetClientEntity(rainstorm_getivengineclient()->GetLocalPlayer( ) ));
    // Huge Credits: Casual_Hacker, I had copied all the code he provided.
    ClientClass* pEntCC = pEnt->GetClientClass();
    const char* ccName = pEntCC->GetName();
	//fprintf(logfile, "%s\n", ccName);
    if ( strcmp(ccName, "CFuncRespawnRoomVisualizer") == 0 || strcmp(ccName, "CTFMedigunShield") == 0 )
		//|| strcmp(ccName,"CFuncAreaPortalWindow") == 0  || strcmp(ccName, "CBaseEntity"))
    {
        return false;
    }

    if ( pEnt == me )
    {
        return false;
    }

    return true;
}
TraceType_t TriggerbotTraceFilter::GetTraceType() const
{
    return TRACE_EVERYTHING;
}
TriggerbotTraceFilter global_filter;
extern "C" ITraceFilter *get_tracefilter(C_BaseEntity *me) {
	return &global_filter;
}
extern "C" const char *c_baseentity_getclassname(C_BaseEntity *ent) {
	ClientClass* pEntCC = ent->GetClientClass();
	const char* ccName = pEntCC->GetName();
	return ccName;
}
extern "C" bool ismousedown() {
	return GetAsyncKeyState(VK_LBUTTON) != 0;
}

extern "C" int calc_seed_from_command_number(int commandnum) {
	return MD5_PseudoRandom( commandnum ) & 0x7fffffff;
}
extern "C" int icliententitylist_get_highest_entity_index(IClientEntityList *entlist) {
	return entlist->GetHighestEntityIndex();
}
bool IKeyValuesDumpContextAsText::KvBeginKey( KeyValues *pKey, int nIndentLevel )
{
	if ( pKey )
	{
		return
			KvWriteIndent( nIndentLevel ) &&
			KvWriteText( pKey->GetName() ) &&
			KvWriteText( " {\n" );
	}
	else
	{
		return
			KvWriteIndent( nIndentLevel ) &&
			KvWriteText( "<< NULL >>\n" );
	}
}

bool IKeyValuesDumpContextAsText::KvWriteValue( KeyValues *val, int nIndentLevel )
{
	if ( !val )
	{
		return
			KvWriteIndent( nIndentLevel ) &&
			KvWriteText( "<< NULL >>\n" );
	}

	if ( !KvWriteIndent( nIndentLevel ) )
		return false;

	if ( !KvWriteText( val->GetName() ) )
		return false;

	if ( !KvWriteText( " " ) )
		return false;

	switch ( val->GetDataType() )
	{
	case KeyValues::TYPE_STRING:
		{
			if ( !KvWriteText( val->GetString() ) )
				return false;
		}
		break;

	case KeyValues::TYPE_INT:
		{
			int n = val->GetInt();
			char *chBuffer = ( char * ) stackalloc( 128 );
			V_snprintf( chBuffer, 128, "int( %d = 0x%X )", n, n );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;

	case KeyValues::TYPE_FLOAT:
		{
			float fl = val->GetFloat();
			char *chBuffer = ( char * ) stackalloc( 128 );
			V_snprintf( chBuffer, 128, "float( %f )", fl );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;

	case KeyValues::TYPE_PTR:
		{
			void *ptr = val->GetPtr();
			char *chBuffer = ( char * ) stackalloc( 128 );
			V_snprintf( chBuffer, 128, "ptr( 0x%p )", ptr );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;

	case KeyValues::TYPE_WSTRING:
		{
			wchar_t const *wsz = val->GetWString();
			int nLen = V_wcslen( wsz );
			int numBytes = nLen*2 + 64;
			char *chBuffer = ( char * ) stackalloc( numBytes );
			V_snprintf( chBuffer, numBytes, "%ls [wstring, len = %d]", wsz, nLen );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;

	case KeyValues::TYPE_UINT64:
		{
			uint64 n = val->GetUint64();
			char *chBuffer = ( char * ) stackalloc( 128 );
			V_snprintf( chBuffer, 128, "u64( %lld = 0x%llX )", n, n );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;

	default:
		break;
		{
			int n = val->GetDataType();
			char *chBuffer = ( char * ) stackalloc( 128 );
			V_snprintf( chBuffer, 128, "??kvtype[%d]", n );
			if ( !KvWriteText( chBuffer ) )
				return false;
		}
		break;
	}

	return KvWriteText( "\n" );
}

bool IKeyValuesDumpContextAsText::KvEndKey( KeyValues *pKey, int nIndentLevel )
{
	if ( pKey )
	{
		return
			KvWriteIndent( nIndentLevel ) &&
			KvWriteText( "}\n" );
	}
	else
	{
		return true;
	}
}

bool IKeyValuesDumpContextAsText::KvWriteIndent( int nIndentLevel )
{
	int numIndentBytes = ( nIndentLevel * 2 + 1 );
	char *pchIndent = ( char * ) stackalloc( numIndentBytes );
	memset( pchIndent, ' ', numIndentBytes - 1 );
	pchIndent[ numIndentBytes - 1 ] = 0;
	return KvWriteText( pchIndent );
}


bool CKeyValuesDumpContextAsDevMsg::KvBeginKey( KeyValues *pKey, int nIndentLevel )
{
	static ConVarRef r_developer( "developer" );
	if ( r_developer.IsValid() && r_developer.GetInt() < m_nDeveloperLevel )
		// If "developer" is not the correct level, then avoid evaluating KeyValues tree early
		return false;
	else
		return IKeyValuesDumpContextAsText::KvBeginKey( pKey, nIndentLevel );
}

bool CKeyValuesDumpContextAsDevMsg::KvWriteText( char const *szText )
{
	if ( m_nDeveloperLevel > 0 )
	{
		DevMsg( m_nDeveloperLevel, "%s", szText );
	}
	else
	{
		Msg( "%s", szText );
	}
	return true;
}
bool KeyValues::Dump( IKeyValuesDumpContext *pDump, int nIndentLevel /* = 0 */ )
{
	if ( !pDump->KvBeginKey( this, nIndentLevel ) )
		return false;

	// Dump values
	for ( KeyValues *val = this ? GetFirstValue() : NULL; val; val = val->GetNextValue() )
	{
		if ( !pDump->KvWriteValue( val, nIndentLevel + 1 ) )
			return false;
	}

	// Dump subkeys
	for ( KeyValues *sub = this ? GetFirstTrueSubKey() : NULL; sub; sub = sub->GetNextTrueSubKey() )
	{
		if ( !sub->Dump( pDump, nIndentLevel + 1 ) )
			return false;
	}

	return pDump->KvEndKey( this, nIndentLevel );
}