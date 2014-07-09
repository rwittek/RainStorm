#include <windows.h>

extern "C" void rainstorm_entrypt();

extern "C" BOOL APIENTRY DllMain( HINSTANCE hInstance, DWORD dwReasonOfCall, LPVOID lpReserved ) {
	if (dwReasonOfCall == DLL_PROCESS_ATTACH) rainstorm_entrypt();
	return true;
}
extern "C" void * engine_getptr();
