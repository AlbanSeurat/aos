#include <stddef.h>
#include <stdint.h>
#include <util.h>
#include <raspi.h>
#include "uart.h"
#include "kprintf.h"
#include "timer.h"

// we put the kernel just before the device and we will protect the last 4k bytes to prevent buffer overrun
char * kernel = DEVICE_BASE - SECTION_SIZE;

void main()
{
    uint32_t size = 500, i = 0;

    uart0_init();

    again:
    uart_putc('R');
    uart_putc('B');
    uart_putc('I');
    uart_putc('N');
    uart_putc('6');
    uart_putc('4');
    uart_putc('\r');
    uart_putc('\n');
    // notify raspbootcom to send the kernel
    uart_putc(3);
    uart_putc(3);
    uart_putc(3);
/*

    // read the kernel's size
    size= uart_getc();
    size |= uart_getc()<<8U;
    size |= uart_getc()<<16U;
    size |= uart_getc()<<24U;

    // send negative or positive acknowledge
    if(size < 64 || size > SECTION_SIZE) {
        // size error
        uart_putc('S');
        uart_putc('E');
        goto again;
    }
    uart_putc('O');
    uart_putc('K');
*/

    // read the kernel
    while(i < size) {
        uint8_t value = uart_getc();
        kernel[i++] = value;
    }

    // jump to the new kernel. we must force an absolute address
    asm volatile ("mov x30, %0; ret" :: "r"(kernel));
}