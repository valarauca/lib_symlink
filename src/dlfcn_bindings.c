#include <dlfcn.h>

/*******************************************************************************
 *
 * void* open_lib(2)
 *
 *  Opens a library that can dynmically load symbols
 *
 ******************************************************************************/
void*
open_lib( char* lib_name, int flags) {
   return dlopen(lib_name,flags);
}

/*******************************************************************************
 *
 * char* lib_err(0)
 *
 *  Returns error string associated with current error
 *
 ******************************************************************************/
char*
lib_err() {
  return dlerror();
}

/*******************************************************************************
 *
 * char* lib_close(1)
 *
 *  Closes library
 *
 ******************************************************************************/
int
close_lib(void* handle) {
  return dlclose(handle);
}

/*******************************************************************************
 *
 * void* dlsym(2)
 *
 *  Returns an untyped pointer to the function symbol requested
 *
 ******************************************************************************/
 void*
 load_symbol(void* handle, char* func_name) {
   return dlsym(handle,func_name);
 }
