#include <stddef.h>
#include <stdint.h>
#include "util.h"
#include "uart.h"
#include "kprintf.h"

#define CORE0_TIMER_IRQCNTL (void*)0x40000040

void timer_init() {

    register unsigned int cntfrq;

    asm volatile ("mrs %0, CNTFRQ_EL0" : "=r" (cntfrq));
    asm volatile ("msr cntv_tval_el0, %0" :: "r" (cntfrq));

    mmio_write(CORE0_TIMER_IRQCNTL, 0x08);
    asm volatile ("msr cntv_ctl_el0, %0" :: "r" (1));
}

void irq_tick_timer() {
    register unsigned int cntfrq;

    asm volatile ("mrs %0, CNTFRQ_EL0" : "=r" (cntfrq));
    asm volatile ("msr cntv_tval_el0, %0" :: "r" (cntfrq));
}
