#ifndef _CHEAT_MENU_H_
#define _CHEAT_MENU_H_
//===================================================================================
#include <windows.h>
//==================================================================================
typedef struct CMenuItems_t
{
	char szTitle[30];
	float* value;
	float flMin;
	float flMax;
	float flStep;
};
//===================================================================================
class CCheatMenu
{
public:
	void Render( void );
	void DrawMenu( void );
	int	AddItem(int nIndex, char szTitle[128], float* value, float flMin, float flMax, float flStep);

//private:
	bool bMenuActive;
	int iMenuIndex;
	int iMenuItems;
	CMenuItems_t pMenu[120];

	
};
//===================================================================================
extern CCheatMenu gCheatMenu;
//===================================================================================
#endif