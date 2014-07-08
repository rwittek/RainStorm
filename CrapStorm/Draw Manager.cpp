#include "Draw Manager.h"
//===================================================================================
CDrawManager gDrawManager;
//===================================================================================
void CDrawManager::DrawRect( int x, int y, int w, int h, DWORD dwColor )
{
	g_pSurface->DrawSetColor( RED(dwColor), GREEN(dwColor), BLUE(dwColor), ALPHA(dwColor) );
	g_pSurface->DrawFilledRect( x, y, x + w, y + h );
}
//===================================================================================
void CDrawManager::OutlineRect( int x, int y, int w, int h, DWORD dwColor )
{
	g_pSurface->DrawSetColor( RED(dwColor), GREEN(dwColor), BLUE(dwColor), ALPHA(dwColor) );
	g_pSurface->DrawOutlinedRect( x, y, x + w, y + h );
}
//===================================================================================
void CDrawManager::DrawBox( Vector vOrigin, int r, int g, int b, int alpha, int box_width, int radius )
{
	Vector vScreen;

	if( !WorldToScreen( vOrigin, vScreen ) )
		return;

	int radius2 = radius<<1;

	//outlines
	OutlineRect( vScreen.x - radius + box_width, vScreen.y - radius + box_width, radius2 - box_width, radius2 - box_width, COLORCODE( 0, 0, 0, 255 ) ); //outer
	OutlineRect( vScreen.x - radius - 1, vScreen.y - radius - 1, radius2 + ( box_width + 2 ), radius2 + ( box_width + 2 ), COLORCODE( 0, 0, 0, 255 ) ); //inner


	//actual lines
	DrawRect( vScreen.x - radius + box_width, vScreen.y - radius, radius2 - box_width, box_width,COLORCODE( r, g, b, alpha )); //top
	DrawRect( vScreen.x - radius, vScreen.y + radius, radius2, box_width,COLORCODE( r, g, b, alpha )); //bottom
	DrawRect( vScreen.x - radius, vScreen.y - radius, box_width, radius2,COLORCODE( r, g, b, alpha )); //left
	DrawRect( vScreen.x + radius, vScreen.y - radius, box_width, radius2 + box_width, COLORCODE( r, g, b, alpha ) ); //right
}
//===================================================================================
bool CDrawManager::ScreenTransform( const Vector &point, Vector &screen )
{
	float w;
	const VMatrix &worldToScreen = g_pEngine->WorldToScreenMatrix( );
	screen.x = worldToScreen[0][0] * point[0] + worldToScreen[0][1] * point[1] + worldToScreen[0][2] * point[2] + worldToScreen[0][3];
	screen.y = worldToScreen[1][0] * point[0] + worldToScreen[1][1] * point[1] + worldToScreen[1][2] * point[2] + worldToScreen[1][3];
	w		 = worldToScreen[3][0] * point[0] + worldToScreen[3][1] * point[1] + worldToScreen[3][2] * point[2] + worldToScreen[3][3];
	screen.z = 0.0f;

	bool behind = false;

	if( w < 0.001f )
	{
		behind = true;
		screen.x *= 100000;
		screen.y *= 100000;
	}
	else
	{
		behind = false;
		float invw = 1.0f / w;
		screen.x *= invw;
		screen.y *= invw;
	}
	return behind;
}
//===================================================================================
bool CDrawManager::WorldToScreen( const Vector &vOrigin, Vector &vScreen )
{
	const VMatrix &worldToScreen = g_pEngine->WorldToScreenMatrix( );

	float w = worldToScreen[3][0] * vOrigin[0] + worldToScreen[3][1] * vOrigin[1] + worldToScreen[3][2] * vOrigin[2] + worldToScreen[3][3]; //Calculate the angle in compareson to the player's camera.
	vScreen.z = 0; //Screen doesn't have a 3rd dimension.

	if( w > 0.001 ) //If the object is within view.
	{
		float fl1DBw = 1 / w; //Divide 1 by the angle.
		vScreen.x = (gScreenSize.iScreenWidth / 2) + ( 0.5 * ((worldToScreen[0][0] * vOrigin[0] + worldToScreen[0][1] * vOrigin[1] + worldToScreen[0][2] * vOrigin[2] + worldToScreen[0][3]) * fl1DBw) * gScreenSize.iScreenWidth + 0.5); //Get the X dimension and push it in to the Vector.
		vScreen.y = (gScreenSize.iScreenHeight / 2) - ( 0.5 * ((worldToScreen[1][0] * vOrigin[0] + worldToScreen[1][1] * vOrigin[1] + worldToScreen[1][2] * vOrigin[2] + worldToScreen[1][3]) * fl1DBw) * gScreenSize.iScreenHeight + 0.5); //Get the Y dimension and push it in to the Vector.
		return true;
	}

	return false;
	/*
	if( ScreenTransform( vOrigin , vScreen ) == false )
	{
		float x = gScreenSize.iScreenWidth / 2;
		float y = gScreenSize.iScreenHeight / 2;
		x += 0.5 * vScreen.x * gScreenSize.iScreenWidth + 0.5;
		y -= 0.5 * vScreen.y * gScreenSize.iScreenHeight + 0.5;
		vScreen.x = x;
		vScreen.y = y;
		return true;
	}
	return false;
	*/
}
//===================================================================================