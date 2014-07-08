#include <Windows.h>
#include "SDK.h"
#include "Memory Tools.h"
//===================================================================================
CMemoryTools gMemoryTools;
//===================================================================================
DWORD	CMemoryTools::dwFindPattern ( DWORD dwAddress, DWORD dwSize, BYTE* pbMask, char* szMask )
{
	for( DWORD i = NULL; i < dwSize; i++ )
		if( bDataCompare( (BYTE*) ( dwAddress + i ), pbMask, szMask ) )
			return (DWORD)( dwAddress + i );

	return 0;
}
//===================================================================================
bool	CMemoryTools::bDataCompare( const BYTE* pData, const BYTE* bMask, const char* szMask )
{
	for( ; *szMask; ++szMask, ++pData, ++bMask )
		if( *szMask == 'x' && *pData != *bMask )
			return false;

	return ( *szMask ) == NULL;
}
//===================================================================================
void	CMemoryTools::MemCopy( PVOID dwAddress, const void *dwBytes, int iSize )
{
	DWORD dwOldProtect, dwNewProtect;
	VirtualProtect(dwAddress, iSize, PAGE_EXECUTE_READWRITE, &dwOldProtect);
	RtlCopyMemory( (PVOID)dwAddress, dwBytes, iSize);
	VirtualProtect(dwAddress, iSize, dwOldProtect, &dwNewProtect); 
}
//===================================================================================
void	CMemoryTools::RemovePEHeader( DWORD dwModuleBase )
{
	PIMAGE_DOS_HEADER pDosHeader = (PIMAGE_DOS_HEADER)dwModuleBase;
	PIMAGE_NT_HEADERS pNTHeader = (PIMAGE_NT_HEADERS)( (DWORD)pDosHeader + (DWORD)pDosHeader->e_lfanew );

	if(pDosHeader->e_magic != IMAGE_DOS_SIGNATURE)
		return;

	if(pNTHeader->Signature != IMAGE_NT_SIGNATURE)
		return;

	if(pNTHeader->FileHeader.SizeOfOptionalHeader)
	{
		DWORD dwProtect;
		WORD Size = pNTHeader->FileHeader.SizeOfOptionalHeader;
		VirtualProtect( (PVOID)dwModuleBase, Size, PAGE_EXECUTE_READWRITE, &dwProtect );
		RtlZeroMemory( (PVOID)dwModuleBase, Size );
		VirtualProtect( (PVOID)dwModuleBase, Size, dwProtect, &dwProtect );
	}
}
//===================================================================================
bool CMemoryTools::bHideModule( HMODULE hmModule )
{
	pProcessModuleInfo pModuleInfo;

	__asm 
	{
		MOV EAX, FS:[18h]
		MOV EAX, [EAX+30h]
		MOV EAX, [EAX+0Ch]
		MOV pModuleInfo, EAX
	}


	pModuleInfoNode pFirstModuleNode = (pModuleInfoNode)pModuleInfo->LoadOrder.Blink;
	pModuleInfoNode pModuleNode = pFirstModuleNode;

	do 
	{
		if( pModuleNode->BaseAddress == hmModule ) 
		{
			pModuleNode->LoadOrder.Blink->Flink = pModuleNode->LoadOrder.Flink;
			pModuleNode->LoadOrder.Flink->Blink = pModuleNode->LoadOrder.Blink;

			pModuleNode->InitOrder.Blink->Flink = pModuleNode->InitOrder.Flink;
			pModuleNode->InitOrder.Flink->Blink = pModuleNode->InitOrder.Blink;

			pModuleNode->MemoryOrder.Blink->Flink = pModuleNode->MemoryOrder.Flink;
			pModuleNode->MemoryOrder.Flink->Blink = pModuleNode->MemoryOrder.Blink;

			pModuleNode->HashTable.Blink->Flink = pModuleNode->HashTable.Flink;
			pModuleNode->HashTable.Flink->Blink = pModuleNode->HashTable.Blink;

			RtlZeroMemory( pModuleNode->FullPath.Buffer, pModuleNode->FullPath.Length );
			RtlZeroMemory( pModuleNode, sizeof( ModuleInfoNode ) );

			return true;
		}

		pModuleNode = ( pModuleInfoNode )pModuleNode->LoadOrder.Blink;

	}	while(pModuleNode != pFirstModuleNode);

	return false;
}
//===================================================================================