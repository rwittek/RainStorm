#ifndef _COLORMANAGER_H_
#define _COLORMANAGER_H_
//===================================================================================
#include <Windows.h>
#include <vector>
//===================================================================================
using namespace std;
//===================================================================================
typedef struct color_t
{
	string strName;
	DWORD dwColor;
};
//===================================================================================
#define COLORCODE(r,g,b,a)((DWORD)((((r)&0xff)<<24)|(((g)&0xff)<<16)|(((b)&0xff)<<8)|((a)&0xff)))
#define RED(COLORCODE)	((int) ( COLORCODE >> 24) )
#define BLUE(COLORCODE)	((int) ( COLORCODE >> 8 ) & 0xFF )
#define GREEN(COLORCODE)	((int) ( COLORCODE >> 16 ) & 0xFF )
#define ALPHA(COLORCODE)	((int) COLORCODE & 0xFF )
#define RGBA(COLORCODE) RED( COLORCODE ), GREEN( COLORCODE ), BLUE( COLORCODE ), ALPHA( COLORCODE )
//===================================================================================
class CColorManager
{
	public:
		void Initialize( void );
		void SetColor( const char* pszName, DWORD dwNewColor );
		DWORD dwGetColor( int iIndex );

	private:
		void AddColor( const char* pszName, DWORD dwNewColor );
		std::vector< color_t > m_Colors;
};
//===================================================================================
extern CColorManager gColorManager;
//===================================================================================
#endif