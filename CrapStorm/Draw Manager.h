#ifndef _DRAW_MANAGER_H_
#define _DRAW_MANAGER_H_
//===================================================================================
#include "SDK.h"
#include "Panels.h"
#include "Color Manager.h"
//===================================================================================
class CDrawManager
{

public:
	void DrawBox( Vector vOrigin, int r, int g, int b, int alpha, int box_width, int radius );
	void DrawRect( int x, int y, int w, int h, DWORD dwColor );
	void OutlineRect( int x, int y, int w, int h, DWORD dwColor );
	
	bool WorldToScreen( const Vector &vOrigin, Vector &vScreen );

	void DrawCrosshair( int iValue )
	{
		int m_iScreenWidth = gScreenSize.iScreenWidth;
		int	m_iScreenHeight = gScreenSize.iScreenHeight;

		DWORD dwRed = COLORCODE( 255, 50, 0, 255 );
		DWORD dwWhite = COLORCODE( 255, 255, 255, 255 );

		switch( iValue )
		{
		case 1:
			DrawRect( ( m_iScreenWidth / 2) - 25, m_iScreenHeight / 2, 50, 1, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) - 25, 1, 50, dwWhite );
			DrawRect( ( m_iScreenWidth / 2) - 7, m_iScreenHeight / 2, 14, 1, dwRed );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) - 7, 1, 14, dwRed );
			break;
		case 2:
			DrawRect( m_iScreenWidth / 2 - 14, (m_iScreenHeight / 2), 9, 1, dwWhite );
			DrawRect( m_iScreenWidth / 2 +5,   (m_iScreenHeight / 2), 9, 1, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) - 14, 1, 9, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) +  5, 1, 9, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2)     , 1, 1, dwWhite );

			break;
		case 3:
			DrawRect( m_iScreenWidth / 2 - 14, (m_iScreenHeight / 2), 9,2, dwWhite );
			DrawRect( m_iScreenWidth / 2 +6,   (m_iScreenHeight / 2), 9,2, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) - 14, 2,9, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2) +  7, 2,9, dwWhite );
			DrawRect( m_iScreenWidth / 2, (m_iScreenHeight / 2)     , 2,2, dwWhite );
			break;
		case 4:
			DrawRect( ( m_iScreenWidth / 2 ) - 2, ( m_iScreenHeight / 2 ) - 2, 4, 4, dwWhite );
			break;
		}
	}

private:
		bool ScreenTransform( const Vector &point, Vector &screen );
};
//===================================================================================
extern CDrawManager gDrawManager;
//===================================================================================
#endif