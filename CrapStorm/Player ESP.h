#ifndef _PLAYER_ESP_H_
#define _PLAYER_ESP_H_

class CPlayerESP
{
public:
	void DrawPlayerESP( int iIndex );
	void DrawWorldESP( int iIndex );	
	void DrawMyESP();
};
extern CPlayerESP gPlayerESP;

#endif