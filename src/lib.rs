extern crate libc;
use libc::c_char;
use std::ffi::CString;

extern {
    fn open_lib(lib_name: *const c_char, flags: u64) -> u64;
    fn lib_err() -> *mut c_char;
    fn close_lib(handle: u64) -> u64;
    fn load_symbol(handle: u64,func_name: *const c_char) -> u64;
}


// A type that represents a library.
//
// This type allows for symlinking against x86_64 Linux C/Rust symbols.
pub struct SymLinkLibrary {
    load_flags: u64,
    handle: u64
}
impl SymLinkLibrary {
    pub fn new() -> SymLinkLibrary {
        SymLinkLibrary {
            load_flags: 0u64,
            handle: 0u64
        }
    }
    // Creates a new loader object

    pub fn load_lazy(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 1;
        s
    }
    // Same as setting RTLD_LAZY
    //
    // Perform lazy binding.  Only resolve symbols as the code that
    // references them is executed.  If the symbol is never
    // referenced, then it is never resolved.  (Lazy binding is
    // performed only for function references; references to
    // variables are always immediately bound when the shared object
    // is loaded.)  Since glibc 2.1.1, this flag is overridden by the
    // effect of the LD_BIND_NOW environment variable.
    pub fn load_now(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 2;
        s
    }
    // Same as setting RTLD_NOW
    //
    // If this value is specified, or the environment variable
    // LD_BIND_NOW is set to a nonempty string, all undefined symbols
    // in the shared object are resolved before dlopen() returns.  If
    // this cannot be done, an error is returned.
    pub fn load_global(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 256;
        s
    }
    // Same as setting RTLD_GLOBAL
    //
    // The symbols defined by this shared object will be made
    // available for symbol resolution of subsequently loaded shared
    // objects.
    pub fn load_local(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 0;
        s
    }
    // Same as setting RTLD_LOCAL
    //
    // This is the converse of RTLD_GLOBAL, and the default if
    // neither flag is specified.  Symbols defined in this shared
    // object are not made available to resolve references in
    // subsequently loaded shared objects.
    pub fn load_nodelete(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 4096;
        s
    }
    // Same as Setting RTLD_NODELETE
    //
    // Do not unload the shared object during dlclose().
    // Consequently, the object's static variables are not
    // reinitialized if the object is reloaded with dlopen() at a
    // later time.
    pub fn load_noload(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 4;
        s
    }
    // Same as setting RTLD_NOLOAD
    //
    // Don't load the shared object.  This can be used to test if the
    // object is already resident (dlopen() returns NULL if it is
    // not, or the object's handle if it is resident).  This flag can
    // also be used to promote the flags on a shared object that is
    // already loaded.  For example, a shared object that was
    // previously loaded with RTLD_LOCAL can be reopened with
    // RTLD_NOLOAD | RTLD_GLOBAL.
    pub fn load_deepbind(self) -> SymLinkLibrary {
        let mut s = self;
        s.load_flags |= 8;
        s
    }
    // Same as setting RTLD_DEEPBIND
    //
    // Place the lookup scope of the symbols in this shared object
    // ahead of the global scope.  This means that a self-contained
    // object will use its own symbols in preference to global
    // symbols with the same name contained in objects that have
    // already been loaded.
    pub fn open<I:Into<Vec<u8>>>(self,path: I)-> Result<SymLinkLibrary,String> {
        let mut s = self;
        let cstr = match CString::new(path) {
            Ok(x) => x,
            Err(_) => return Err("Null byte was in file path. This isn't required".to_string())
        };
        s.handle = unsafe{ open_lib(cstr.as_ptr(),s.load_flags) };
        if s.handle == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err("Error occured while opening library AND while geting error message.".to_string())
            };
        }
        Ok(s)
    }
    // Opens Library
    //
    // As user has populated the flags they want. This consumes the current flags and opens that
    // library for linking.
    pub fn load_0x1<O,Func:Fn()->O,S:Into<Vec<u8>>>(&self,func_name: S) -> Result<Box<Func>,String>{
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err("Null byte was in file path. This isn't required".to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err("Error occured while loading symbol AND while geting error message.".to_string())
            };
        }
        let f: Box<Func> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
    // Load a void function with 1 return value.
    pub fn load_1x1<I,O,Func:Fn(I)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String> {
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err("Null byte was in file path. This isn't required".to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err("Error occured while loading symbol AND while geting error message.".to_string())
            };
        }
        let f: Box<Func> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
    // Loads a function which consumes 1 item off the stack, and places 1 item on the stack
    pub fn load_2x1<I1,I2,O,Func:Fn(I1,I2)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String> {
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err("Null byte was in file path. This isn't required".to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err("Error occured while loading symbol AND while geting error message.".to_string())
            };
        }
        let f: Box<Func> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
    // Loads a function which consumes 2 item's off the stack, and places 1 item on the stack
    pub fn load_3x1<I1,I2,I3,O,Func:Fn(I1,I2,I3)->O,S:Into<Vec<u8>>>(&self,func_name:S) -> Result<Box<Func>,String> {
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err("Null byte was in file path. This isn't required".to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err("Error occured while loading symbol AND while geting error message.".to_string())
            };
        }
        let f: Box<Func> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
    // Loads a function which consumes 3 item's off the stack, and places 1 item on the stack
}
impl Drop for SymLinkLibrary {
    fn drop(&mut self) {
        let _ = unsafe { close_lib( self.handle ) };
    }
}
