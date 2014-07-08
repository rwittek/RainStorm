#ifndef _PANELS_H_
#define _PANELS_H_
//===================================================================================
#include <Windows.h>
#include "SDK.h"
//===================================================================================
typedef struct CScreenSize_t
{
	int iScreenHeight;
	int iScreenWidth;

} CScreenSize;
//===================================================================================
class CDrawPanel : public vgui::Panel
{
	typedef vgui::Panel BaseClass;

public:
	CDrawPanel( vgui::VPANEL parent );
	//virtual void Paint( );
};
//===================================================================================
class CDrawLayer
{
private:
	CDrawPanel*  pDrawPanel;

public:
	CDrawLayer ( void );
	~CDrawLayer ( void );

	void Create ( vgui::VPANEL parent );
	void Destroy( void );
};
//===================================================================================
extern CScreenSize gScreenSize;	
void BuildPanelHook( PDWORD* pdwPanel );
//===================================================================================
#endif