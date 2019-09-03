#include <stddef.h>
#include <stdint.h>
#include <util.h>
#include "kprintf.h"
#include "timer.h"

extern uint32_t exception_vector[];

#define CORE0_IRQ_SOURCE    (void*)0x40000060

void exceptions_init(void) {
    asm volatile("msr vbar_el1, %[base]" :: [base] "r" (exception_vector));
}

void enable_irq() {
    asm volatile("msr daifclr,#2");
}

void disable_irq() {
    asm volatile("msr daifset,#2");
}

void c_irq_handler() {
    disable_irq();

    int irq_source =  mmio_read(CORE0_IRQ_SOURCE);
    if(irq_source & 0x08) {
        irq_tick_timer();
    }
    enable_irq();
}

void hang() {
    kprintf("panic\n");
    wfe: asm volatile("wfe");
    goto wfe;
}
