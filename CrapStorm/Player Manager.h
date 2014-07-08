#ifndef _LOCAL_PLAYER_H_
#define _LOCAL_PLAYER_H_

#include "SDK.h"
#include "Client.h"
#include "Extra Functions.h"

#define me 1337

#define M_RADPI 57.295779513082f
#define SQUARE( a ) a*a

class CSetupPlayer
{
public:

	inline void Initialize( unsigned int nIndex ) 
	{ 
		m_flDistance	= 9999.9f;
		m_nIndex		= nIndex;
	}

	inline IClientEntity *Ent( void ) const
	{
		return g_pEntList->GetClientEntity( m_nIndex );
	}

	inline C_BaseEntity *BaseEnt( void ) const
	{
		IClientEntity* pBaseEntity = this->Ent();
		return dynamic_cast<C_BaseEntity*>(pBaseEntity);
	}

	inline C_BasePlayer *BasePlayer( void ) const
	{
		return ToBasePlayer( BaseEnt( ) );
	}

	inline IClientRenderable *RenderPlayer( void ) const
	{
		return Ent( )->GetClientRenderable( );
	}

	inline C_BaseCombatCharacter* CombatCharacter( void ) const
	{
		return ToBaseCombatCharacter( BaseEnt( ) );
	}

// 	inline C_BaseCombatWeapon* CombatWeapon( void ) const
// 	{
// 		return GetBaseCombatActiveWeapon(BaseEnt());
// 	}

	inline void MakeVector( const QAngle& qAngle, QAngle& qVector ) 
	{ 
		float pitch = 0.0f; 
		float yaw = 0.0f; 
		float tmp = 0.0f;            

		pitch = (float)(qAngle[0] * M_PI/180.0f); 
		yaw = (float)(qAngle[1] * M_PI/180.0f); 
		tmp = (float)cos(pitch);

		qVector[0] = (float) (-tmp * -cos(yaw)); 
		qVector[1] = (float) (sin(yaw)*tmp);
		qVector[2] = (float) -sin(pitch);
	}

	inline bool IsAlive( void )
	{
		return BaseEnt( )->IsAlive( );
	}

	inline int GetTeam( void )
	{
		return *MakePtr(int*,BaseEnt(),gOffsets.m_iTeamNum);
	}

	inline void GetCenterOrg( Vector &vCenterOrg )
	{
		Vector vMin, vMax;
		RenderPlayer( )->GetRenderBounds( vMin, vMax );
		vCenterOrg = Ent( )->GetAbsOrigin( );
		vCenterOrg.z += (vMin.z + vMax.z) / 2.0f;
	}

	inline Vector GetAimOrg( void ) { return m_vAimOrg; }
	inline int GetHitbox( void ) { return m_iHitbox; }
	inline Vector &GetEyePos( void ) { return m_vEyePos; }
	inline player_info_s &GetPlayerInfo( void ) { return m_pInfo; }
	inline char *GetWeaponName( void ) const { return m_pszWeapon; }
	inline float GetAimFOV( void ) { return m_flFOV; }

	inline void SetAimOrg( const Vector &vAimOrg ) { m_vAimOrg = vAimOrg; }
	inline void SetHitbox( const int &iHitbox ) { m_iHitbox = iHitbox; }
	inline void SetEyePos( const Vector &vEyePos ) { m_vEyePos = vEyePos; }
	inline void SetPlayerInfo( const player_info_s &PlayerInfo ) { m_pInfo = PlayerInfo; }
	inline void SetWeaponName( char *pszWeapon ) { m_pszWeapon = pszWeapon; }
	inline void SetAimFOV( const float &flFOV ) { m_flFOV = flFOV; }

private:

	Vector m_vAimOrg;
	Vector m_vEyePos;

	player_info_s m_pInfo;

	char *m_pszWeapon;

	int m_iHitbox;
	int m_nIndex;

	float m_flDistance;
	float m_flFOV;

	void CalcAngle( const Vector& vSource, const Vector& vDest, QAngle& vAngles )
	{
		double delta[3] = { (vSource[0]-vDest[0]), (vSource[1]-vDest[1]), (vSource[2]-vDest[2]) };
		double hyp = sqrt( delta[0]*delta[0] + delta[1]*delta[1] );

		vAngles[0] = (float) (atan(delta[2]/hyp) * M_RADPI);
		vAngles[1] = (float) (atan(delta[1]/delta[0]) * M_RADPI);
		vAngles[2] = 0.0f;

		if( delta[0] >= 0.0f ) 
			vAngles[1] += 180.0f;
	}

};

class CPlayers
{
public:
	inline CPlayers( void ) 
	{
		pPlayers = new CSetupPlayer[65];

		for( unsigned int i = 0; i <= 64; ++i )
			pPlayers[i].Initialize( i );
	}

	inline ~CPlayers( )
	{
		delete [] pPlayers;
	}

	inline CSetupPlayer& operator [] ( unsigned int i ) const
	{
		if( i == me )
			return pPlayers[ g_pEngine->GetLocalPlayer( ) ];

		else if( i > 64 || i <= 0 ) 
			return pPlayers[0];

		return pPlayers[i];
	}

private:
	CSetupPlayer *pPlayers;
}; 

extern CPlayers gPlayers;

#endif