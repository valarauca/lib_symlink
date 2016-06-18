#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>

int
main(void) 
{
   printf("RTLD_LAZY: %d\n",RTLD_LAZY);
   printf("RTLD_NOW: %d\n",RTLD_NOW);
   printf("RTLD_GLOBAL: %d\n", RTLD_GLOBAL);
   printf("RTLD_LOCAL: %d\n",RTLD_LOCAL);
   printf("RTLD_NODELETE: %d\n", RTLD_NODELETE);
   printf("RTLD_NOLOAD: %d\n", RTLD_NOLOAD);
   printf("RTLD_DEEPBIND: %d\n", RTLD_DEEPBIND);
}
