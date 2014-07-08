#include "SDK.h"
#include "Draw Manager.h"
#include "File Manager.h"
#include "VMT Hook.h"
#include "Font Manager.h"
#include "Client.h"
#include "PaintTraverse.h"
#include "Panel.h"

CPaintTraverse PaintTraverseClass;
//===================================================================================
CScreenSize gScreenSize;
CVMTHookManager* g_pPanelHook = NULL;

//===================================================================================
CDrawLayer::CDrawLayer ( void )
{
	pDrawPanel = NULL;
}
//===================================================================================
CDrawLayer::~CDrawLayer ( void )
{
	Destroy();
}
//===================================================================================
void CDrawLayer::Create( vgui::VPANEL parent )
{
	pDrawPanel = new CDrawPanel ( parent );
}
//===================================================================================
void CDrawLayer::Destroy( void )
{
	if ( pDrawPanel )
	{
		ExitProcess( 0 );
		pDrawPanel->SetParent( (vgui::Panel *)NULL );
		delete pDrawPanel;
		pDrawPanel = NULL;
	}
}
//===================================================================================
CDrawPanel::CDrawPanel( vgui::VPANEL parent ) : BaseClass( NULL, "staticDrawOurPanel" ){}
//===================================================================================
static void __stdcall hook_PaintTraverse( vgui::VPANEL vguiPanel, bool forceRepaint, bool allowForce )
{
	//g_pEngine->ClientCmd_Unrestricted("echo test");
	g_pPanelHook->UnHook();
	g_pIPanel->PaintTraverse( vguiPanel, forceRepaint, allowForce );
	g_pPanelHook->ReHook();

	static unsigned int vguiMatSystemTopPanel;

	if (vguiMatSystemTopPanel == NULL)
	{
		const char* szName = g_pIPanel->GetName(vguiPanel);
		if( szName[0] == 'M' && szName[3] == 'S' ) //Look for MatSystemTopPanel without using slow operations like strcmp or strstr.
		{
			vguiMatSystemTopPanel = vguiPanel;
		}
	}
	if (vguiMatSystemTopPanel == vguiPanel)
	{
		if( g_pEngine->IsLevelMainMenuBackground( ) || g_pEngine->Con_IsVisible( ) || !g_pEngine->IsInGame( ) || !g_pEngine->IsConnected( ) )
			return;

		if( g_pEngine->IsTakingScreenshot( ) )
			return;	

		PaintTraverseClass.Invoke();
	}

	//gDrawManager.DrawCrosshair( gCvars.misc_cross );
}

void BuildPanelHook( PDWORD* pdwPanel )
{
	g_pPanelHook = new CVMTHookManager( (PDWORD*)g_pIPanel );
	g_pPanelHook->dwHookMethod( ( DWORD )hook_PaintTraverse, 41 );
}