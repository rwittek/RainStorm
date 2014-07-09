g++ -mwindows -c wrapper.cpp -o target\deps\wrapper.o;
ar -rcs target\deps\libwrapper.a target\deps\wrapper.o;