#include <windows.h>
#include <tlhelp32.h>
#include <winsock.h>
#include <algorithm>
#include <time.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/timeb.h>
#include <vector>
#include <fstream>
#include <istream>
#include <string.h>

#include "public/cdll_int.h"

extern "C" void rainstorm_init(int log_fd);

DWORD WINAPI startup_thread( LPVOID lpArguments ) {
	FILE *f = fopen("rainstorm_debug.txt", "w");
	rainstorm_init(fileno(f));
	//exit(1);
}

extern "C" BOOL APIENTRY DllMain( HINSTANCE hInstance, DWORD dwReasonOfCall, LPVOID lpReserved ) {
	if ( dwReasonOfCall == DLL_PROCESS_ATTACH ) {
		CreateThread( NULL, 0, (LPTHREAD_START_ROUTINE)startup_thread, NULL, 0, NULL );
	}
	return true;
}

HMODULE GetModuleHandleSafe( const char* pszModuleName )
{
	HMODULE hmModuleHandle = NULL;

	do
	{
		hmModuleHandle = GetModuleHandle( pszModuleName );
		Sleep( 1 );
	}
	while(hmModuleHandle == NULL);

	return hmModuleHandle;
}

extern "C" void * getptr_engine() {
	HMODULE hmEngine = GetModuleHandleSafe( "engine.dll" );
	CreateInterfaceFn EngineFactory = ( CreateInterfaceFn ) GetProcAddress( hmEngine, "CreateInterface" );
	return EngineFactory( VENGINE_CLIENT_INTERFACE_VERSION, NULL );
}

extern "C" void engine_clientcmd(void *engine_ptr, const char *command) {
	((IVEngineClient *) engine_ptr)->ClientCmd(command);
}
