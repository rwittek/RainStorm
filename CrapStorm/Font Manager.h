#ifndef _CFONTMANAGER_H_
#define _CFONTMANAGER_H_
//===================================================================================
#include <Windows.h>
#include "SDK.h"
//===================================================================================
class CFontManager
{
public:
	CFontManager( void )
	{
		m_bInited = false;
	}
	void Initialize( void );
	void DrawString( bool bHUDFont, int x, int y, DWORD dwColor, const char *pszText, ... );
	void DrawString( bool bHUDFont, int x, int y, int r, int g, int b, int a, const char *pszText, ... );
	void CFontManager::DrawString(bool bHUDFont, int x,int y, DWORD dwcolor, wchar_t* unichar);
	unsigned int GetHUDHeight( void ) const;
	unsigned int GetESPHeight( void ) const;
	vgui::HFont GetHUDFont( void ) const;
	vgui::HFont GetESPFont( void ) const;

private:
	unsigned int m_nHUDHeight;
	unsigned int m_nESPHeight;
	vgui::HFont m_HUDFont;
	vgui::HFont m_ESPFont;
	vgui::HFont m_CrossFont;
	bool m_bInited;
};
//===================================================================================
extern CFontManager gFontManager;
//===================================================================================
#endif