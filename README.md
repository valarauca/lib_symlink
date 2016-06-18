# lib_symlink
Platform specific bindings for x86_64 Linux to dlfcn.h 

It currently only supports symbols that modify the stack like

    fn x(0) -> 1
    fn x(1) -> 1
    fn x(2) -> 1
    fn x(3) -> 1
  
If you need more arguments, please submit a pull request.
These are limited by the type/generic system.
I hate saying this but C++ templates would be useful here.

###Example
    let lib = match SymLinkLibrary::new().load_nodelete().load_lazy().open("/lib/your_lib.so") {
        Ok(x) => x,
        Err(e) => panic!("Error loading library: {}", e)
    };
    //typing information needs to be supplied
    let my_func: Box<Fn(i32)->i32> = match lib.load_1x1("you_func_name") {
        Ok(x) => x,
        Err(e) => panic!("Error fetching function: {}", e)
    };

###Documentation:


    SymLinkLibrary {
       ...
    }
  
This type allows for symlinking against x86_64 Linux C/Rust symbols.

    fn load_lazy(self) -> SymLinkLibrary

Same as setting RTLD_LAZY

Perform lazy binding.  Only resolve symbols as the code that
references them is executed.  If the symbol is never
referenced, then it is never resolved.  (Lazy binding is
performed only for function references; references to
variables are always immediately bound when the shared object
is loaded.)  Since glibc 2.1.1, this flag is overridden by the
effect of the LD_BIND_NOW environment variable.

    fn load_now(self) -> SymLinkLibrary
    
Same as setting RTLD_NOW

If this value is specified, or the environment variable
LD_BIND_NOW is set to a nonempty string, all undefined symbols
in the shared object are resolved before dlopen() returns.  If
this cannot be done, an error is returned.

    fn load_global(self) -> SymLinkLibray
    
Same as setting RTLD_GLOBAL

The symbols defined by this shared object will be made
available for symbol resolution of subsequently loaded shared
objects.

    fn load_local(self) -> SymLinkLibrary
    
Same as setting RTLD_LOCAL

This is the converse of RTLD_GLOBAL, and the default if
neither flag is specified.  Symbols defined in this shared
object are not made available to resolve references in
subsequently loaded shared objects.

    fn load_nodelete(self) -> SymLinkLibrary
    
Same as Setting RTLD_NODELETE

Do not unload the shared object during dlclose().
Consequently, the object's static variables are not
reinitialized if the object is reloaded with dlopen() at a
later time.

    fn load_noload(self) -> SymLinkLibrary
    
Same as setting RTLD_NOLOAD

Don't load the shared object.  This can be used to test if the
object is already resident (dlopen() returns NULL if it is
not, or the object's handle if it is resident).  This flag can
also be used to promote the flags on a shared object that is
already loaded.  For example, a shared object that was
previously loaded with RTLD_LOCAL can be reopened with
RTLD_NOLOAD | RTLD_GLOBAL.

    fn load_deepbind(self) -> SymLinkLibrary
    
Same as setting RTLD_DEEPBIND

Place the lookup scope of the symbols in this shared object
ahead of the global scope.  This means that a self-contained
object will use its own symbols in preference to global
symbols with the same name contained in objects that have
already been loaded.

    fn open<I:Into<Vec<u8>>>(self,path: I)-> Result<SymLinkLibrary,String>
    
Opens the library.

Following the programmer poulating flags, this executes the dlopen call.


    fn load_0x1<O,Func:Fn()->O,S:Into<Vec<u8>>>(&self,func_name: S) -> Result<Box<Func>,String>
    fn load_1x1<I,O,Func:Fn(I)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String>
    fn load_2x1<I1,I2,O,Func:Fn(I1,I2)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String>
    fn load_3x1<I1,I2,I3,O,Func:Fn(I1,I2,I3)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String>
    
These functions actually return function pointers... Or Boxed Functions which can then be called.
  
