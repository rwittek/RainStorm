#ifndef _EXTRA_FUNCTIONS_H_
#define _EXTRA_FUNCTIONS_H_
//===================================================================================
#include "SDK.h"
//===================================================================================
typedef struct CSafeUserCmd_t 
{ 
	CUserCmd pCommand; 
	DWORD dwUnk0; 
} CSafeUserCmd;  
//===================================================================================
typedef struct CVerifyFuncs_t
{
	void *SaveUserCmd0;     
	void *SaveUserCmd1;     
} CVerifyFuncs;
//===================================================================================
C_BaseCombatWeapon * GetBaseCombatActiveWeapon ( C_BaseEntity* pEntity );

const char* GetWeaponClass( C_BaseCombatWeapon* pWeapon );

void Spinbot( CUserCmd* pCommand );

bool bTraceToPlayer( void );

void GetWorldSpaceCenter( C_BaseEntity* pBaseEntity, Vector& vWorldSpaceCenter );

float flGetDistance( Vector vOrigin, Vector vLocalOrigin );

QAngle qGetPunchAngle( void );

const char* szGetTF2Class( int iClass );

void RemoveSniperScope( void );

//void UnprotectCvars( void );

void LowerCase( char *szString );
//===================================================================================
extern CVerifyFuncs gVerifyFuncs;
//===================================================================================
#endif/