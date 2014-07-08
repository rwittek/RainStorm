#include "PaintTraverse.h"
#include "Panels.h"
#include "Cheat Menu.h"
#include "Utilities.h"
#include "Control Variables.h"
#include "Player ESP.h"
#include "Extra Functions.h"
#include "Color Manager.h"
#include "Font Manager.h"
#include "Client.h"
#include "Player Manager.h"
#include "Memory Tools.h"

void CPaintTraverse::Invoke()
{
	g_pEngine->GetScreenSize( gScreenSize.iScreenWidth, gScreenSize.iScreenHeight );

	gFontManager.DrawString( true , gScreenSize.iScreenWidth / 2, gScreenSize.iScreenHeight / 4, gColorManager.dwGetColor(3), "CrapStorm"); //Remove this if you want.

	if( gCheatMenu.bMenuActive )
		gCheatMenu.DrawMenu( );

#if COMPILE_ESP
	for( int iIndex = 0; iIndex < g_pEntList->GetHighestEntityIndex( ); iIndex++ )
	{
 		gPlayerESP.DrawPlayerESP( iIndex );
 		gPlayerESP.DrawWorldESP( iIndex );
	}
#endif
#if COMPILE_MISC
	if (gCvars.misc_taunt)
	{
		int* m_nPlayerCond = MakePtr(int*,gPlayers[me].BaseEnt(),gOffsets.m_Shared + gOffsets.m_nPlayerCond);
		if (*m_nPlayerCond & TFCond_Taunting)
			*m_nPlayerCond &= ~TFCond_Taunting;
	}
#endif
	// Put your code below
}
