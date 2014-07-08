#include <Windows.h>
#include "Font Manager.h"
#include "Color Manager.h"
//===================================================================================
CFontManager gFontManager;
//===================================================================================
void CFontManager::Initialize( void )
{
	if ( g_pSurface == NULL || m_bInited )
		return;

	m_bInited = true;

	g_pSurface->AddCustomFontFile("TF2 Build","resource\\tf2build.ttf");

	m_nHUDHeight = 11;
	m_HUDFont =  g_pSurface->CreateFont( );
	g_pSurface->SetFontGlyphSet( m_HUDFont, "TF2 Build", m_nHUDHeight, 100, 0, 0, 0x200 );

	m_nESPHeight = 14;
	m_ESPFont = g_pSurface->CreateFont( );
	g_pSurface->SetFontGlyphSet( m_ESPFont, "TF2 Build", m_nESPHeight, 400, 0, 0, 0x300 );
}
//===================================================================================
void CFontManager::DrawString( bool bHUDFont, int x, int y, DWORD dwColor, const char *pszText, ... )
{
	if( pszText == NULL )
		return;

	va_list va_alist;
	char szBuffer[1024] = { '\0' };
	wchar_t szString[1024] = { '\0' };

	va_start( va_alist, pszText );
	vsprintf( szBuffer, pszText, va_alist );
	va_end( va_alist );

	wsprintfW( szString, L"%S", szBuffer );
	DrawString(bHUDFont,x,y, dwColor,szString);
}
//==================================================================================
void CFontManager::DrawString(bool bHUDFont, int x,int y, DWORD dwColor, wchar_t* unichar)
{
	g_pSurface->DrawSetTextPos( x, y );
	g_pSurface->DrawSetTextFont( bHUDFont == true ? m_HUDFont : m_ESPFont );
	g_pSurface->DrawSetTextColor( RED(dwColor), GREEN(dwColor), BLUE(dwColor), ALPHA(dwColor) );
	g_pSurface->DrawPrintText( unichar, wcslen( unichar ) );
}
//===================================================================================
void CFontManager::DrawString( bool bHUDFont, int x, int y, int r, int g, int b, int a, const char *pszText, ... )
{
	if( pszText == NULL )
		return;

	va_list va_alist;
	char szBuffer[1024] = { '\0' };
	wchar_t szString[1024] = { '\0' };

	va_start( va_alist, pszText );
	vsprintf( szBuffer, pszText, va_alist );
	va_end( va_alist );

	wsprintfW( szString, L"%S", szBuffer );

	g_pSurface->DrawSetTextPos( x, y );
	g_pSurface->DrawSetTextFont( bHUDFont == true ? m_HUDFont : m_ESPFont );
	g_pSurface->DrawSetTextColor( r, g, b, a );
	g_pSurface->DrawPrintText( szString, wcslen( szString ) );
}
//===================================================================================
unsigned int CFontManager::GetHUDHeight( void ) const
{
	return m_nHUDHeight;
}
//===================================================================================
unsigned int CFontManager::GetESPHeight( void ) const
{
	return m_nESPHeight;
}
//===================================================================================
vgui::HFont CFontManager::GetHUDFont( void ) const
{
	return m_HUDFont;
}
//===================================================================================
vgui::HFont CFontManager::GetESPFont( void ) const
{
	return m_ESPFont;
}
//===================================================================================