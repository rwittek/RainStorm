#ifndef _WORLD_OBJECTS_H_
#define _WORLD_OBJECTS_H_
//===================================================================================
#include "SDK.h"
//===================================================================================
class CObject : public C_BaseEntity
{
// public:
// 	int m_iHealth;
// 	int m_iMaxHealth;
// 	bool m_bHasSapper;
// 	int m_iObjectType;
// 	int m_bBuilding;
// 	bool m_bPlacing;
// 	bool m_bCarried;
// 	bool m_bCarryDeploy;
// 	bool m_bMiniBuilding;
// 	float m_flPercentageConstructed;
// 	int m_fObjectFlags;
// 	C_BaseEntity* m_hBuiltOnEntity;
// 	bool m_bDisabled;
// 	C_BaseEntity* m_m_hBuilder;
// 	Vector m_vecBuildMaxs;
// 	Vector m_vecBuildMins;
// 	int m_iDesiredBuildRotations;
// 	bool m_bServerOverridePlacement;
// 	int m_iUpgradeLevel;
// 	int m_iUpgradeMetal;
// 	int m_iHighestUpgradeLevel;
// 	int m_iObjectMode;
// 	bool m_bDisposableBuilding;

};
//===================================================================================
class CObjectDispenser : public CObject
{
// public:
// 	int m_iState;
// 	int m_iAmmoMetal;                //0xE60 - 0xE64
};
//===================================================================================
class CObjectSentryGun : public CObject
{
// public:
// 	int m_iAmmoShells;
// 	int m_iAmmoRockets;
// 	int m_iState;
// 	bool m_bPlayerControlled;
// 	int m_nShieldLevel;
// 	bool  m_bShielded;
// 	/* Actually EHANDLE */
// 	C_BaseEntity* m_hEnemy; 
// 	C_BaseEntity* m_hAutoAimTarget;
public:
	inline char* GetStateString(int __state)
	{
		switch( __state )
		{
		case 1:
			{
				return "Idle";
			}
		case 2:
			{
				return "Attacking";
			}
		case 3:
			{
				return "Upgrading";
			}
		}
		return "Unknown";
	}
};
//===================================================================================
class CObjectTeleporter : public CObject
{
// public:
// 	unsigned char    ucUnknown011[ 0x8 ];        //0xE40 - 0xE48
// 	int                m_iState;                    //0xE48 - 0xE4C
// 	int                m_iUnknown012;                //0xE4C - 0xE50
// 	float            m_flRechargeTime;            //0xE50 - 0xE54
// 	int                m_iTimesUsed;                //0xE54 - 0xE58
// 	float            m_flYawToExit;                //0xE58 - 0xE5C
public:
	inline char* GetStateString(int __state)
	{
		switch( __state )
		{
		case 1:
			{
				return "Idle";
			}
		case 2:
			{
				return "Active";
			}
		case 4:
			{
				return "Teleporting";
			}
		case 6:
			{
				return "Charging";
			}
		}

		return "Unknown";
	}
};
//===================================================================================
CObject *ToBaseObject( C_BaseEntity *pBaseEntity )
{
	return reinterpret_cast< CObject *>( pBaseEntity );
}
//===================================================================================
#endif