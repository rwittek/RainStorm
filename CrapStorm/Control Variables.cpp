#include <Windows.h>
#include <stdio.h>
#include <fstream>
#include "SDK.h"
#include "Control Variables.h"
#include "File Manager.h"

//===================================================================================
CControlVariables gCvars;
//===================================================================================
void CControlVariables::Initialize( void )
{
	misc_menu_x = 300;
	misc_menu_y = 75;
	misc_menu_w = 200;
}
//===================================================================================
void CControlVariables::Save( void )
{
#if COMPILE_AIMBOT
	gFileManager.WriteFloat( "Aimbot" , "Aimbot" , aim_bot );
	gFileManager.WriteFloat( "Aimbot" , "Aim Key", aim_key );
	gFileManager.WriteFloat( "Aimbot" , "Aim FOV", aim_fov );
	gFileManager.WriteFloat( "Aimbot" , "Auto Aim", aim_auto_aim );
	gFileManager.WriteFloat( "Aimbot" , "Auto Fire",aim_auto_shoot);
	gFileManager.WriteFloat( "Aimbot" ,"Aim Spot", aim_spot );
	gFileManager.WriteFloat( "Aimbot" ,"Aim Lock", aim_lock );
	//gFileManager.WriteFloat( "Aimbot" ,"Smooth Aim", aim_smooth );
	gFileManager.WriteFloat( "Aimbot" ,"Trigger Bot", aim_triggerbot );
	gFileManager.WriteFloat( "Aimbot" ,"Prediction", aim_prediction );
#endif

#if COMPILE_ESP
	gFileManager.WriteFloat( "ESP"	  , "Draw Team", esp_team );
	gFileManager.WriteFloat( "ESP"	  ,"Name ESP", esp_name );
	gFileManager.WriteFloat( "ESP"	  , "Health ESP", esp_health );
	gFileManager.WriteFloat( "ESP"	  , "Distance ESP", esp_dist );
	gFileManager.WriteFloat( "ESP"	  , "Box ESP", esp_box );
	gFileManager.WriteFloat( "ESP"	  , "Class ESP", esp_class );
	gFileManager.WriteFloat( "ESP"	  , "Object ESP", esp_object );
#endif

#if COMPILE_MISC
	gFileManager.WriteFloat( "Misc"	  , "Bunny Hop", misc_bunnyhop );
	gFileManager.WriteFloat( "Misc"	  , "Taunt Slide", misc_taunt );
	gFileManager.WriteFloat( "Misc"	  , "Auto Backstab", misc_auto_bs );
	gFileManager.WriteFloat( "Misc"	  , "Remove Cloak", misc_spy_cloak);
	gFileManager.WriteFloat( "Misc"	  , "Remove disguise", misc_spy_disg);
#endif
}
//===================================================================================
void CControlVariables::Load( void )
{
#if COMPILE_AIMBOT
	aim_bot        =	gFileManager.ReadFloat( "Aimbot", "Aimbot" , aim_bot );
	aim_key		   =	gFileManager.ReadFloat( "Aimbot", "Aim Key", aim_key );
	aim_fov		   =	gFileManager.ReadFloat( "Aimbot", "Aim FOV", aim_fov );
	aim_auto_aim   =	gFileManager.ReadFloat( "Aimbot", "Auto Aim", aim_auto_aim );
	aim_auto_shoot =	gFileManager.ReadFloat("Aimbot","Auto Fire",aim_auto_shoot);
	aim_spot	   =	gFileManager.ReadFloat( "Aimbot", "Aim Spot", aim_spot );
	aim_lock	   =	gFileManager.ReadFloat( "Aimbot", "Aim Lock", aim_lock );
	//aim_smooth	   =	gFileManager.ReadFloat( "Aimbot", "Smooth Aim", aim_smooth );
	aim_triggerbot =	gFileManager.ReadFloat( "Aimbot", "Trigger Bot", aim_triggerbot );
	aim_prediction =	gFileManager.ReadFloat( "Aimbot", "Prediction", aim_prediction );
#endif

#if COMPILE_ESP
	esp_team	   =    gFileManager.ReadFloat( "ESP", "Draw Team", esp_team );
	esp_name       =	gFileManager.ReadFloat( "ESP", "Name ESP", esp_name );
	esp_health	   =	gFileManager.ReadFloat( "ESP", "Health ESP", esp_health );
	esp_dist	   =	gFileManager.ReadFloat( "ESP", "Distance ESP", esp_dist );
	esp_box        =	gFileManager.ReadFloat( "ESP", "Box ESP", esp_box );
	esp_class      =	gFileManager.ReadFloat( "ESP", "Class ESP", esp_class );
	esp_object     =	gFileManager.ReadFloat( "ESP", "Object ESP", esp_object );
#endif

#if COMPILE_MISC
	misc_bunnyhop  =   gFileManager.ReadFloat( "Misc","Bunny Hop",misc_bunnyhop);
	misc_taunt	   =   gFileManager.ReadFloat( "Misc", "Taunt Slide", misc_taunt );
	misc_auto_bs   =   gFileManager.ReadFloat( "Misc", "Auto Backstab", misc_auto_bs );
	misc_spy_cloak =   gFileManager.ReadFloat( "Misc","Remove Cloak", misc_spy_cloak);
	misc_spy_disg  =   gFileManager.ReadFloat( "Misc","Remove disguise", misc_spy_disg);
#endif
}
//===================================================================================
