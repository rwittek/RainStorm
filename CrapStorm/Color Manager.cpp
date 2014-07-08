#include "Color Manager.h"
//===================================================================================
CColorManager gColorManager;
//===================================================================================
void CColorManager::Initialize( void )
{
	AddColor( "objectives",	COLORCODE( 255, 255, 255, 255 ) );		// 0
	AddColor( "teamone",	COLORCODE( 0, 128,   255, 255 ) );		// 1
	AddColor( "teamtwo",	COLORCODE( 255,   128,   0, 255 ) );	// 2
	AddColor( "teamthree",	COLORCODE( 0, 128, 255, 255 ) );		// 3
	AddColor( "teamfour",	COLORCODE( 255, 128, 0, 255 ) );		// 4
	AddColor( "background", COLORCODE(  30,  30,  30, 128 ) );		// 5
	AddColor( "lines",		COLORCODE( 255, 120,   0, 255 ) );		// 6
	AddColor( "outlines",	COLORCODE( 255, 100, 000, 255 ) );		// 7
	AddColor( "selected",	COLORCODE( 255, 100, 000, 255 ) );		// 8
	AddColor( "hud_text",	COLORCODE( 255, 255, 255, 255 ) );		// 9
	AddColor( "cross",		COLORCODE( 255, 255, 255, 255 ) );		// 10
	AddColor( "cross_sec",	COLORCODE( 255,   0,   0, 255 ) );		// 11
	AddColor( "invis_team2",COLORCODE( 110, 180,  255, 255 ) );		// 12
	AddColor( "invis_team3",COLORCODE( 255,   0,   0, 255 ) );		// 13
	AddColor( "menu_on",	COLORCODE(  0,  128, 255, 255 ) );		// 14
	AddColor( "menu_off",	COLORCODE( 255,   255,   255, 255 ) );	// 15
	AddColor( "aim_target",	COLORCODE(	 0, 255,   0, 255 ) );		// 16
	AddColor( "black",		COLORCODE(	 0,	  0,   0, 255 ) );		// 17
}
//===================================================================================
DWORD CColorManager::dwGetColor( int iIndex )
{
	if ( iIndex < 0 || ( unsigned int )iIndex > m_Colors.size() )
	{
		return 0xFFFFFFFF;
	}

	return m_Colors[ iIndex ].dwColor;
}
//===================================================================================
void CColorManager::SetColor( const char* pszName, DWORD dwNewColor )
{
	for ( unsigned int i = 0; i < m_Colors.size(); i++ )
	{
		if ( stricmp( m_Colors[i].strName.c_str(), pszName ) == 0 )
		{
			m_Colors[i].dwColor = dwNewColor;
			break;
		}
	}
}
//===================================================================================
void CColorManager::AddColor( const char* pszName, DWORD dwNewColor )
{
	color_t color;
	color.dwColor = dwNewColor;
	color.strName = pszName;

	m_Colors.push_back( color );
}
//===================================================================================