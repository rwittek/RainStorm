#ifndef _AIMBOT_H_
#define _AIMBOT_H_

#include <Windows.h>
#include "SDK.h"


#define	CHAR_TEX_CONCRETE		'C'
#define CHAR_TEX_METAL			'M'
#define CHAR_TEX_DIRT			'D'
#define CHAR_TEX_VENT			'V'
#define CHAR_TEX_GRATE			'G'
#define CHAR_TEX_TILE			'T'
#define CHAR_TEX_SLOSH			'S'
#define CHAR_TEX_WOOD			'W'
#define CHAR_TEX_COMPUTER		'P'
#define CHAR_TEX_GLASS			'Y'
#define CHAR_TEX_FLESH			'F'
#define CHAR_TEX_BLOODYFLESH	'B'
#define CHAR_TEX_CLIP			'I'
#define CHAR_TEX_ANTLION		'A'
#define CHAR_TEX_ALIENFLESH		'H'
#define CHAR_TEX_FOLIAGE		'O'
#define CHAR_TEX_SAND			'N'
#define CHAR_TEX_PLASTIC		'L'

class CAimbot
{
public:
	CAimbot( );

	bool bGetTeam( int iIndex );
	bool bIsTargetSpot( int iIndex );
	bool bIsValidEntity( int iIndex );
	bool bIsValidTarget( int iIndex );

	int	iGetTarget	( void );
	void FindTarget( void );
	void AimAtTarget();
	bool bIsVisible( const Vector& vecAbsStart, const Vector& vecAbsEnd, C_BaseEntity* pBaseEnt );
	void CalculateTick( void );
	void PredictTarget( Vector &vOrigin, Vector &vPredicted );
	void DropTarget( void );
	
	void CalcAngle( const Vector& vSource, const Vector& vDest, QAngle& vAngles );

	float CalcFOVAngle( QAngle qAngle, Vector vSrc, Vector vDst );

private:

	float flGetFOV( Vector vOrigin );
	float flGetDistance( Vector vOrigin );

	QAngle qAimAngles;

	int m_nTarget;
	float g_fTick;

	Vector vTargetOrg;
	Vector vPrediction;
	Vector vMin, vMax;
	Vector vecEyePosition, vecBackupEyePosition, vecLastEyePosition;

	//Vector vTmpBones;

	matrix3x4_t pBoneToWorld;

	float flBestDist;
};

extern CAimbot gAimbot;

#endif