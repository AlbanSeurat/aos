#include <stddef.h>
#include <stdint.h>
#include "uart.h"
#include "printf.h"

#if defined(__cplusplus)
extern "C" /* Use C linkage for kernel_main. */
#endif
void main(uint32_t r0, uint32_t r1, uint32_t atags)
{
    unsigned long el;

    // Declare as unused
    (void) r0;
    (void) r1;
    (void) atags;

    uart0_init();

    printf("Hello World!\n");

    asm volatile ("mrs %0, CurrentEL" : "=r" (el));

    printf("Current EL is: %x\n", (el>>2)&3);


}