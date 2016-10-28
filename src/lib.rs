//Copyright 2016 William Cody Laeder
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.




//!SymLink
//!
//!This crate provides a simple interface to interact with dlfcn.h bindings
//!on POSIX-y systems (MacOS, *BSD, Linux, etc.). This library should look
//!and feel like OpenOptions in the standard library.
//!
//!     uselib_symlinkj:: 
//!
//!The FreeBSD extensions to dlfcn.h are not currently supported, but they will
//!likely be added in the future. 
//!
//!For additional reading [FreeBSD
//!ManPage](https://www.freebsd.org/cgi/man.cgi?query=dlopen&sektion=3) [Linux
//!ManPage](https://linux.die.net/man/3/dlopen) [OSX
//!Manpage](https://developer.apple.com/legacy/library/documentation/Darwin/Reference/ManPages/man3/dlopen.3.html)
//!The documentation I'm providing will nearly be verbatim to what is found in
//!those documents, but there maybe some minor platform differences you should 
//!familiarize yourself with. 


extern crate libc;
use libc::c_char;
use std::ffi::CString;

extern {
    fn open_lib(lib_name: *const c_char, flags: u64) -> u64;
    fn lib_err() -> *mut c_char;
    fn close_lib(handle: u64) -> u64;
    fn load_symbol(handle: u64,func_name: *const c_char) -> u64;
}


const NULLBYTE: &'static str = "Null byte was found in file path"; 
const DBLERR: &'static str = "Error occured while loading symbol AND while geting error message.";

macro_rules! define_ret {
    ($name: ident,$($args: ident),*) => {
        pub fn $name<$($args,)*F,O,S>(&self, func_name: S) -> Result<Box<F>,String>
        where
            O: Sized,
            $(
                $args: Sized,
            )*
            F: Fn($($args,)*)->O,
            S: Into<Vec<u8>>
    {
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err(NULLBYTE.to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err(DBLERR.to_string())
            };
        }
        let f: Box<F> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
}}
macro_rules! define_void {
    ($name: ident, $($args: ident),*) => {
        pub fn $name<$($args,)*F,S>(&self, func_name: S) -> Result<Box<F>,String>
        where
            $(
                $args: Sized,
            )*
            F: Fn($($args,)*),
            S: Into<Vec<u8>>
    {
        let cstr = match CString::new(func_name) {
            Ok(x) => x,
            Err(_) => return Err(NULLBYTE.to_string())
        };
        let ptr = unsafe{ load_symbol(self.handle, cstr.as_ptr()) };
        if ptr == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err(DBLERR.to_string())
            };
        }
        let f: Box<F> = unsafe{ std::mem::transmute(ptr) };
        Ok(f)
    }
    }
}


///Dynamic Library Handle
///
///This type allows for symlinking against .so files. It should be noted that
///if you load C++ or ObjC or _any_ _language_ _that_ _is_ _not_ _C_ This crate
///is not doing the name mangling, you have too.
pub struct DynLib {
    load_flags: u64,
    handle: u64
}
impl DynLib {
    ///Creates a new loader object. 
    pub fn new() -> DynLib {
        DynLib {
            load_flags: 0u64,
            handle: 0u64
        }
    }
    ///Same as setting RTLD_LAZY
    ///
    ///Perform lazy binding.  Only resolve symbols as the code that
    ///references them is executed.  If the symbol is never
    ///referenced, then it is never resolved.  (Lazy binding is
    ///performed only for function references; references to
    ///variables are always immediately bound when the shared object
    ///is loaded.)  Since glibc 2.1.1, this flag is overridden by the
    ///effect of the LD_BIND_NOW environment variable.
    pub fn load_lazy(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 1;
        s
    }

    ///Same as setting RTLD_NOW
    ///
    ///If this value is specified, or the environment variable
    ///LD_BIND_NOW is set to a nonempty string, all undefined symbols
    ///in the shared object are resolved before dlopen() returns.  If
    ///this cannot be done, an error is returned.
    pub fn load_now(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 2;
        s
    }

    ///Same as setting RTLD_GLOBAL
    ///
    ///The symbols defined by this shared object will be made
    ///available for symbol resolution of subsequently loaded shared
    ///objects.
    pub fn load_global(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 2;
        s
    }
    
    ///Same as setting RTLD_LOCAL
    ///
    ///This is the converse of RTLD_GLOBAL, and the default if
    ///neither flag is specified.  Symbols defined in this shared
    ///object are not made available to resolve references in
    ///subsequently loaded shared objects.
    pub fn load_local(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 0;
        s
    }
    
    ///Same as Setting RTLD_NODELETE
    ///
    ///Do not unload the shared object during dlclose().
    ///Consequently, the object's static variables are not
    ///reinitialized if the object is reloaded with dlopen() at a
    ///later time.
    pub fn load_nodelete(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 4096;
        s
    }
    
    ///Same as setting RTLD_NOLOAD
    ///
    ///Don't load the shared object.  This can be used to test if the
    ///object is already resident (dlopen() returns NULL if it is
    ///not, or the object's handle if it is resident).  This flag can
    ///also be used to promote the flags on a shared object that is
    ///already loaded.  For example, a shared object that was
    ///previously loaded with RTLD_LOCAL can be reopened with
    ///RTLD_NOLOAD | RTLD_GLOBAL.
    pub fn load_noload(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 4;
        s
    }
    
    ///Same as setting RTLD_DEEPBIND
    ///
    ///Place the lookup scope of the symbols in this shared object
    ///ahead of the global scope.  This means that a self-contained
    ///object will use its own symbols in preference to global
    ///symbols with the same name contained in objects that have
    ///already been loaded.
    pub fn load_deepbind(self) -> DynLib {
        let mut s = self;
        s.load_flags |= 8;
        s
    }
    
    ///Opens Library
    ///
    ///This will consume all the flags set by the developer and open the
    ///specificed .so file.
    pub fn open<S>(self,path: S)-> Result<DynLib,String>
        where
            S: Into<Vec<u8>>
    {
        let mut s = self;
        let cstr = match CString::new(path) {
            Ok(x) => x,
            Err(_) => return Err(NULLBYTE.to_string())
        };
        s.handle = unsafe{ open_lib(cstr.as_ptr(),s.load_flags) };
        if s.handle == 0 {
            let cstr = unsafe { CString::from_raw( lib_err() ) };
            match cstr.into_string() {
                Ok(x) => return Err(x),
                Err(_) => return Err(DBLERR.to_string())
            };
        }
        Ok(s)
    }
    

    ///Load function that takes 1 arg, and returns something
    define_ret!(ret_1, I); 

    ///Load function that takes 2 arg, and returns something
    define_ret!(ret_2, I1, I2);
    
    ///Load function that takes 3 arg, and returns something
    define_ret!(ret_3, I1, I2, I3);
    
    ///Load function that takes 4 arg, and returns something
    define_ret!(ret_4, I1, I2, I3, I4);
    
    ///Load function that takes 5 arg, and returns something
    define_ret!(ret_5, I1, I2, I3, I4, I5);
    
    ///Load function that takes 6 arg, and returns something
    define_ret!(ret_6, I1, I2, I3, I4, I5, I6);
    
    ///Load function that takes 7 arg, and returns something
    define_ret!(ret_7, I1, I2, I3, I4, I5, I6, I7);
    
    ///Load function that takes 8 arg, and returns something
    define_ret!(ret_8, I1, I2, I3, I4, I5, I6, I7, I8);
   
 
    ///Load void function that takes 1 args
    define_void!(void_1, I); 
    
    ///Load void function that takes 2 args
    define_void!(void_2, I1, I2);
    
    ///Load void function that takes 3 args
    define_void!(void_3, I1, I2, I3);
    
    ///Load void function that takes 4 args
    define_void!(void_4, I1, I2, I3, I4);
    
    ///Load void function that takes 5 args
    define_void!(void_5, I1, I2, I3, I4, I5);
    
    ///Load void function that takes 6 args
    define_void!(void_6, I1, I2, I3, I4, I5, I6);
    
    ///Load void function that takes 7 args
    define_void!(void_7, I1, I2, I3, I4, I5, I6, I7);
    
    ///Load void function that takes 8 args
    define_void!(void_8, I1, I2, I3, I4, I5, I6, I7, I8);
}
impl Drop for DynLib {
    fn drop(&mut self) {
        let _ = unsafe { close_lib( self.handle ) };
    }
}
