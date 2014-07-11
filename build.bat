del target\deps\libwrapper.a
del target\deps\wrapper.o
cl /MT /c /EHsc /I  public /I public\tier0 /I  public\tier1 /I  public\common /I  public\client /I  public\shared /I  public\shared\sdk /I  public\vstdlib /I  public\client\sdk /I  publlic\inputsystem /I  public\vgui_controls /I  public\vgui wrapper.cpp /Fotarget\deps\wrapper.o


rustc --crate-type=staticlib -o target/deps/rainstorm.a src/rainstorm.rs -L target/deps -C link-args="-mwindows -static"
link /dll /force:multiple /out:target\rainstorm.dll target\deps\wrapper.o target\deps\rainstorm.a public\ImportLibrarys\bitmap.lib public\ImportLibrarys\nvtristrip.lib public\ImportLibrarys\tier1.lib public\ImportLibrarys\vstdlib.lib public\ImportLibrarys\choreoobjects.lib public\ImportLibrarys\particles.lib public\ImportLibrarys\tier2.lib public\ImportLibrarys\vtf.lib public\ImportLibrarys\dmxloader.lib public\ImportLibrarys\raytrace.lib public\ImportLibrarys\tier3.lib public\ImportLibrarys\mathlib.lib public\ImportLibrarys\steam_api.lib public\ImportLibrarys\vgui_controls.lib public\ImportLibrarys\matsys_controls.lib public\ImportLibrarys\tier0.lib public\ImportLibrarys\vmpi.lib libmingwex.a libmingw32.a libcmt.lib