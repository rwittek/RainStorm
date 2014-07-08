
#ifndef _SDK_H_
#define _SDK_H_
//===================================================================================
#define CLIENT_DLL
//===================================================================================
#pragma warning( disable : 4311 )
#pragma warning( disable : 4312 )
#pragma warning( disable : 4541 )
#pragma warning( disable : 4267 )
#pragma warning( disable : 4183 )
//===================================================================================
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
//#include "Weapon List.h"
#include "Build Config.h"
#define SECURITY_WIN32
#define WIN32_LEAN_AND_MEAN

#pragma optimize("gsy",on)

//#include "Console.h"
#include <Security.h>
#include <shlwapi.h>
//#include "public\client\vgui_grid.h"
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

//===================================================================================
#define IN_ATTACK		(1 << 0)
#define IN_JUMP			(1 << 1)
#define IN_DUCK			(1 << 2)
#define IN_FORWARD		(1 << 3)
#define IN_BACK			(1 << 4)
#define IN_USE			(1 << 5)
#define IN_CANCEL		(1 << 6)
#define IN_LEFT			(1 << 7)
#define IN_RIGHT		(1 << 8)
#define IN_MOVELEFT		(1 << 9)
#define IN_MOVERIGHT	(1 << 10)
#define IN_ATTACK2		(1 << 11)
#define IN_RUN			(1 << 12)
#define IN_RELOAD		(1 << 13)
#define IN_ALT1			(1 << 14)
#define IN_ALT2			(1 << 15)
#define IN_SCORE		(1 << 16)	// Used by client.dll for when scoreboard is held down
#define IN_SPEED		(1 << 17)	// Player is holding the speed key
#define IN_WALK			(1 << 18)	// Player holding walk key
#define IN_ZOOM			(1 << 19)	// Zoom key for HUD zoom
#define IN_WEAPON1		(1 << 20)	// weapon defines these bits
#define IN_WEAPON2		(1 << 21)	// weapon defines these bits
#define IN_BULLRUSH		(1 << 22)
//===================================================================================

enum TFClassType
{
	TFClass_Unknown = 0,
	TFClass_Scout,
	TFClass_Sniper,
	TFClass_Soldier,
	TFClass_DemoMan,
	TFClass_Medic,
	TFClass_Heavy,
	TFClass_Pyro,
	TFClass_Spy,
	TFClass_Engineer
};

enum TFCond
{
    TFCond_Slowed = (1 << 0), //Toggled when a player is slower than normal.
    TFCond_Zoomed = (1 << 1), //Toggled when a player is zoomed in.
    TFCond_Disguising = (1 << 2), //Toggled when a Spy is disguising. 
    TFCond_Disguised = (1 << 3), //Toggled when a Spy is disguised.
    TFCond_Cloaked = (1 << 4), //Toggled when a Spy is invisible.
    TFCond_Ubercharged = (1 << 5), //Toggled when a player is ÜberCharged.
    TFCond_TeleportedGlow = (1 << 6), //Will activate when someone leaves a teleporter and has glow beneath their feet.
    TFCond_Taunting = (1 << 7), //Activates when a player is taunting.
    TFCond_UberchargeFading = (1 << 8), //Activates when the ÜberCharge is fading.
    TFCond_CloakFlicker = (1 << 9), //When a normal cloak Spy gets bumped into, or a CloakAndDagger spy with no energy is moving.
    TFCond_Teleporting = (1 << 10), //Only activates for a brief second when a player is riding a teleporter; not very useful.
    TFCond_Kritzkrieged = (1 << 11), //When a player has a crit buff from the KrRitzkrieg. (No longer used?)
    TFCond_TmpDamageBonus = (1 << 12), //Unknown what this is for.
    TFCond_DeadRingered = (1 << 13), //Toggled when the player is under reduced damage from the Deadringer.
    TFCond_Bonked = (1 << 14), //Player is under the effects of Bonk! Atomic Punch.
    TFCond_Stunned = (1 << 15), //Player was stunned from a Sandman ball.
    TFCond_Buffed = (1 << 16), //Toggled when a player is within a Buff Banner's range.
    TFCond_Charging = (1 << 17), //Toggled when a Demo Knight charges with the shield.
    TFCond_DemoBuff = (1 << 18), //Toggled when a Demo Knight has heads from the Eyelander.
    TFCond_CritCola = (1 << 19), //Toggled when the player is under the effect of Crit-a-Cola.
    TFCond_InHealRadius = (1 << 20), //Unknown what this is for.
    TFCond_Healing = (1 << 21), //Toggled when someone is being healed by a medic or a dispenser.
    TFCond_OnFire = (1 << 22), //Toggled when a player is on fire.
    TFCond_Overhealed = (1 << 23), //Toggled when a player has >100% health.
    TFCond_Jarated = (1 << 24), //Toggled when a player is hit with a sniper's Jarate.
    TFCond_Bleeding = (1 << 25), //Toggled from Boston Basher/Tribalman's Shiv/Southern Hospitality damage.
    TFCond_DefenseBuffed = (1 << 26), //Toggled when a player is within a Battalion's Backup's range.
    TFCond_Milked = (1 << 27), //Player was hit with a jar of Mad Milk.
    TFCond_MegaHeal = (1 << 28), //Player is under the effect of Quick-Fix charge.
    TFCond_RegenBuffed = (1	 << 29), //Toggled when a player is within a Concheror's range.
    TFCond_MarkedForDeath = (1 << 30), //Player is marked for death by a Fan O'War hit. Effects are similar to TFCond_Jarated.

    TFCondEx_SpeedBuffAlly = (1 << 0), //Toggled when a player gets hit with the disciplinary action.
#ifdef HALLOWEEN
    TFCondEx_HalloweenCritCandy = (1 << 1), //Only for Scream Fortress event maps that drop crit candy.
#endif
    TFCondEx_CritHype = (1 << 4), //Soda Popper crits.
    TFCondEx_CritOnFirstBlood = (1 << 5), //Arena first blood crit buff.
    TFCondEx_CritOnWin = (1 << 6), //End of round crits.
    TFCondEx_CritOnFlagCapture = (1 << 7), //CTF intelligence capture crits.
    TFCondEx_CritOnKill = (1 << 8), //Unknown what this is for.
    TFCondEx_RestrictToMelee = (1 << 9), //Unknown what this is for.
    TFCondEx_PyroCrits = (1 << 12), //Pyro is getting crits from the Mmmph charge.
    TFCondEx_PyroHeal = (1 << 13), //Pyro is being healed from the Mmmph charge and can not be damaged.

    TFCond_Crits = ( TFCond_Kritzkrieged ),
    TFCond_MiniCrits = ( TFCond_Buffed | TFCond_CritCola ),
    TFCondEx_Crits = (
#ifdef HALLOWEEN
                        TFCondEx_HalloweenCritCandy |
#endif
                        TFCondEx_CritOnFirstBlood | TFCondEx_CritOnWin | TFCondEx_CritOnFlagCapture | TFCondEx_CritOnKill | TFCondEx_PyroCrits ),
    TFCondEx_MiniCrits = ( TFCondEx_CritHype ),
	TFCond_IgnoreStates = ( TFCond_Ubercharged | TFCond_Bonked ),
	TFCondEx_IgnoreStates = ( TFCondEx_PyroHeal )
};
#define	HIDEHUD_WEAPONSELECTION		( 1<<0 )	// Hide ammo count & weapon selection
#define	HIDEHUD_FLASHLIGHT			( 1<<1 )	// Hide flashlight energy indication
#define	HIDEHUD_ALL					( 1<<2 )	// Hide the whole HUD
#define HIDEHUD_HEALTH				( 1<<3 )	// Hide health & armor / suit battery
#define HIDEHUD_PLAYERDEAD			( 1<<4 )	// Hide when local player's dead
#define HIDEHUD_NEEDSUIT			( 1<<5 )	// Hide when the local player doesn't have the HEV suit
#define HIDEHUD_MISCSTATUS			( 1<<6 )	// Hide miscellaneous status elements (trains, pickup history, death notices, etc)
#define HIDEHUD_CHAT				( 1<<7 )	// Hide all communication elements (saytext, voice icon, etc)
#define	HIDEHUD_CROSSHAIR			( 1<<8 )	// Hide crosshairs
#define	HIDEHUD_VEHICLE_CROSSHAIR	( 1<<9 )	// Hide vehicle crosshair
#define HIDEHUD_INVEHICLE			( 1<<10 )
#define HIDEHUD_SCOPE				( 1<<11 )
//===================================================================================
/*
#pragma comment ( lib, "public/ImportLibrarys/bitmap.lib" )
#pragma comment ( lib, "public/ImportLibrarys/choreoobjects.lib" )
#pragma comment ( lib, "public/ImportLibrarys/mathlib.lib" )
#pragma comment ( lib, "public/ImportLibrarys/nvtristrip.lib" )
#pragma comment ( lib, "public/ImportLibrarys/raytrace.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier0.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier1.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier2.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier3.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vgui_controls.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vstdlib.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vtf.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vmpi.lib" )
*/
#define MakePtr( Type, dwBase, dwOffset ) ( ( Type )( DWORD( dwBase ) + (DWORD)( dwOffset ) ) )
#define me g_pEngine->GetLocalPlayer()
#define GetLocalPlayerEntity g_pEntList->GetClientEntity(me)

#pragma comment ( lib, "public/ImportLibrarys/tier0.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier1.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier2.lib" )
#pragma comment ( lib, "public/ImportLibrarys/tier3.lib" )
#pragma comment ( lib, "public/ImportLibrarys/mathlib.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vstdlib.lib" )
#pragma comment ( lib, "public/ImportLibrarys/raytrace.lib" )
#pragma comment ( lib, "public/ImportLibrarys/vgui_controls.lib" )

// #pragma pointers_to_members( full_generality, virtual_inheritance )
// #pragma check_stack( off )
//===================================================================================
using namespace std;
//===================================================================================
extern IBaseClientDLL*				g_pClient;
extern IMatSystemSurface*			g_pMatSurface;
extern vgui::ISurface*				g_pSurface;
extern IVEngineClient* 				g_pEngine;
extern IEngineVGui*					g_pEngineVGUI;
extern IVDebugOverlay*				g_pDebugOverlay;
extern IMaterialSystem*				g_pMatSystem;
extern IClientEntityList*			g_pEntList;
extern IVModelInfoClient*			g_pModelInfo;
extern CInput*						g_pInput;
extern CGlobalVarsBase* 			g_pGlobals;
extern IPanel*						g_pIPanel;
extern IEngineTrace*				g_pEngineTrace;
extern ICvar*						g_pCvar;
extern IVRenderView*				g_pRender;
//extern CUserMessages*				g_pUserMessages;
extern IEngineSound*				g_pEngineSound;
extern IPrediction*					g_pPrediction;
extern IPhysicsSurfaceProps*		g_pPhysicAPI;
//===================================================================================
extern ISteamClient*				g_pSteamClient;
extern ISteamFriends*				g_pSteamFriends;
extern ISteamUtils*					g_pSteamUtils;
extern ISteamUser*					g_pSteamUser;
extern ISteamUserStats*				g_pSteamStats;
extern ISteamMatchmaking*			g_pMatchMaking;
extern ISteamMatchmakingServers*	g_pMatchMakingSvr;
extern ISteamApps*					g_pSteamApps;
//===================================================================================
extern CreateInterfaceFn			g_AppSysFactory;
//===================================================================================
extern DWORD						dwWeaponIDToAlias;
//===================================================================================
#define XASSERT( x ) if( !x ) MessageBoxW( 0, L#x, 0, 0 );
#define MakePtr( Type, dwBase, dwOffset ) ( ( Type )( DWORD( dwBase ) + (DWORD)( dwOffset ) ) )

extern CUserCmd* _pCommand;

#endif