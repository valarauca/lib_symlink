extern crate libc;
use libc::{ c_int, c_void, c_char};
use std::ffi::CString;

extern {
    fn open_lib(lib_name: *const c_char, flags: u64) -> u64;
    fn lib_err() -> *const c_char;
    fn close_lib(handle: u64) -> u64;
    fn load_symbol(handle: u64,func_name: *const c_char) -> u64;
}


// A type that represents a library.
//
// This type allows for symlinking against x86_64 Linux C/Rust symbols.
pub struct SymLinkLibrary {
    name: String,
    load_flags: u64,
    handle: u64
}
impl SymLinkLibrary {
    pub fn new() -> SymLinkLibrary {
        SymLinkLibrary {
            name: String::new(),
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
}
