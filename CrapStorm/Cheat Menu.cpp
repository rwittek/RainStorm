#include <windows.h>
#include "Draw Manager.h"
#include "Player Manager.h"
#include "Font Manager.h"
#include "Color Manager.h"
#include "Control Variables.h"
#include "Cheat Menu.h"
#include "Utilities.h"
#include "SDK.h"

CCheatMenu gCheatMenu;	
char* subclass;

int CCheatMenu::AddItem(int nIndex, char szTitle[128], float* value, float flMin, float flMax, float flStep )
{
	strcpy( pMenu[nIndex].szTitle, szTitle );
	pMenu[nIndex].value = value;
	pMenu[nIndex].flMin = flMin;
	pMenu[nIndex].flMax = flMax;
	pMenu[nIndex].flStep = flStep;
	return ( nIndex + 1 );
}

void RGB_DWORD(DWORD &dwColor, BYTE r, BYTE g, BYTE b)
{
  dwColor = (r << 24) | (g << 16) | (b << 8);
}

DWORD dwWhite = NULL;
DWORD dwGreen = NULL;


void CCheatMenu::Render( void )
{
	RGB_DWORD(dwWhite, 255, 255, 255);
	RGB_DWORD(dwGreen, 0, 255, 0);

	int i = 0;


#if COMPILE_AIMBOT
		i = AddItem(i, " - Aim Bot", &gCvars.aim_bot, 0, 1, 1);
		i = AddItem(i, " - Aim Key", &gCvars.aim_key, 0, 4, 1);
		i = AddItem(i, " - Aim FOV", &gCvars.aim_fov, 1, 360, 1);
		i = AddItem(i, " - Auto Fire",&gCvars.aim_auto_shoot,0,1,1);
		i = AddItem(i, " - Auto Aim", &gCvars.aim_auto_aim, 0, 1, 1);
		i = AddItem(i, " - Aim Lock", &gCvars.aim_lock, 0, 1, 1 );
		i = AddItem(i, " - Aim Spot", &gCvars.aim_spot, 1, 5, 1 );
		i = AddItem(i, " - Prediction", &gCvars.aim_prediction, 0, 1, 1 );
		//i = AddItem(i, " - Smooth Aim", &gCvars.aim_smooth, 0, 100, 1);
#endif

#if COMPILE_ESP
	i = AddItem(i, " - Draw Team", &gCvars.esp_team, 0, 1, 1);
	i = AddItem(i, " - Name ESP", &gCvars.esp_name, 0, 1, 1);
	i = AddItem(i, " - Health ESP", &gCvars.esp_health, 0, 1, 1);
	i = AddItem(i, " - Box ESP", &gCvars.esp_box, 0, 1, 1);
	i = AddItem(i, " - Distance ESP", &gCvars.esp_dist, 0, 1, 1);
	i = AddItem(i, " - Class ESP", &gCvars.esp_class, 0, 1, 1);
	i = AddItem(i, " - Object ESP",&gCvars.esp_object,0,1,1);
#endif

#if COMPILE_MISC
	i = AddItem(i, " - Bunny Hop",&gCvars.misc_bunnyhop,0,1,1);
	i = AddItem(i, " - Taunt Slide",&gCvars.misc_taunt,0,1,1);
	i = AddItem(i, " - Auto Backstab",&gCvars.misc_auto_bs,0,1,1);
	i = AddItem(i, " - Remove Cloak",&gCvars.misc_spy_cloak,0,1,1);
	i = AddItem(i, " - Remove Disguise",&gCvars.misc_spy_disg,0,1,1);
#endif

	iMenuItems = i;
}

void CCheatMenu::DrawMenu( void )
{
	int x = gCvars.misc_menu_x,
		xx = x + 105,
		y = gCvars.misc_menu_y,
		w = gCvars.misc_menu_w,
		h = gFontManager.GetHUDHeight( );

	gDrawManager.DrawRect( x , y - ( h + 4 ) , w, iMenuItems * 11 + 21, gColorManager.dwGetColor( 5 ) );
	gDrawManager.OutlineRect( x, y - ( h + 4 ), w, ( h + 4 ) , gColorManager.dwGetColor( 5 ) );
	gDrawManager.OutlineRect( x, y - ( h + 4 ), w, iMenuItems * 11 + 21, gColorManager.dwGetColor( 14 ) );


	for( int i = 0; i < iMenuItems; i++ )
	{
		if( i != iMenuIndex )
		{
			gFontManager.DrawString( true, x + 2, y + (11*i),  gColorManager.dwGetColor( 15 ),  pMenu[i].szTitle);
			gFontManager.DrawString( true, xx, y + (11*i), gColorManager.dwGetColor( 15 ), "%2.2f", pMenu[i].value[0] );
		}
		else
		{
			gDrawManager.DrawRect( x + 1, y + (11*i) , w - 2, h, COLORCODE(255, 255, 255, 80) );
			gFontManager.DrawString( true,  x + 2, y + (11*i), gColorManager.dwGetColor( 14 ),  pMenu[i].szTitle );
			gFontManager.DrawString( true, xx, y + (11*i), gColorManager.dwGetColor( 14 ),  "%2.2f", pMenu[i].value[0] );
		}
	}
}