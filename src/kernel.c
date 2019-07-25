#include <stddef.h>
#include <stdint.h>
#include "uart.h"
#include "kprintf.h"
#include "exc.h"
#include "timer.h"

#if defined(__cplusplus)
extern "C" /* Use C linkage for kernel_main. */
#endif
void main(uint32_t r0, uint32_t r1, uint32_t atags)
{
    unsigned long proc_state, el, daif;

    // Declare as unused
    (void) r0;
    (void) r1;
    (void) atags;

    uart0_init();

    exceptions_init();
    kprintf("Exception set\n");

    asm volatile("mrs %0, sctlr_el1" : "=r" (proc_state));
    kprintf("proc state el1: %b\n", proc_state);

    asm volatile ("mrs %0, CurrentEL" : "=r" (el));
    kprintf("Current EL is: %x\n", (el >> 2) & 3);

    timer_init();
    kprintf("timer set\n");

    enable_irq();
    kprintf("IRQ set\n");

    asm volatile ("mrs %0, DAIF" : "=r" (daif));
    kprintf( "Interrupt mask is : %b\n", daif);

    kprintf("Hello World!\n");

    while(1) {
        asm volatile ("wfi");
    };

}