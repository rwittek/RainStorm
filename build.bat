del target\deps\libwrapper.a
del target\deps\wrapper.o
cl /c /EHsc /I  public /I public\tier0 /I  public\tier1 /I  public\common /I  public\client /I  public\shared /I  public\shared\sdk /I  public\vstdlib /I  public\client\sdk /I  publlic\inputsystem /I  public\vgui_controls /I  public\vgui wrapper.cpp /Fotarget\deps\wrapper.o


rustc --crate-type=staticlib -o target/deps/rainstorm.a src/rainstorm.rs -L target/deps -C link-args="-mwindows -static"
link /dll /force:multiple /out:target\rainstorm.dll target\deps\wrapper.o target\deps\rainstorm.a public\ImportLibrarys\mathlib.lib libmingwex.a libmingw32.a libmsvcr110.a