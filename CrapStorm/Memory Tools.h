#ifndef _MEMORY_TOOLS_H_
#define _MEMORY_TOOLS_H_
//===================================================================================
// Windows
//===================================================================================
typedef struct _UNICODE_STRING {
	USHORT  Length;
	USHORT  MaximumLength;
	PWSTR  Buffer;
} UNICODE_STRING, *PUNICODE_STRING;
//===================================================================================
typedef struct _ModuleInfoNode
{
	LIST_ENTRY LoadOrder;
	LIST_ENTRY InitOrder;
	LIST_ENTRY MemoryOrder;
	HMODULE BaseAddress;		//	Base address AKA module handle
	unsigned long EntryPoint;
	UNICODE_STRING FullPath;
	unsigned int Size;			//	Size of the modules image
	UNICODE_STRING Name;
	unsigned long Flags;
	unsigned short LoadCount;
	unsigned short TlsIndex;
	LIST_ENTRY HashTable;	//	A linked list of any other modules that have the same first letter
	unsigned long Timestamp;
} ModuleInfoNode, *pModuleInfoNode;
//===================================================================================
typedef struct _ProcessModuleInfo
{
	unsigned int Size;			//	Size of a ModuleInfo node?
	unsigned int Initialized;
	HANDLE SsHandle;
	LIST_ENTRY LoadOrder;
	LIST_ENTRY InitOrder;
	LIST_ENTRY MemoryOrder;
} ProcessModuleInfo, *pProcessModuleInfo;
//===================================================================================
// Memory Tools
//===================================================================================
class CMemoryTools
{
public:
	void RemovePEHeader( DWORD dwModuleBase );
	void MemCopy( PVOID dwAddress, const void *dwBytes, int iSize );
	bool bDataCompare( const BYTE* pData, const BYTE* bMask, const char* szMask );
	DWORD dwFindPattern( DWORD dwAddress ,DWORD dwLen, BYTE *bMask, char * szMask );
	bool bHideModule( HMODULE hmModule );
};
//===================================================================================
extern CMemoryTools gMemoryTools;
//===================================================================================
#endif