#include <stddef.h>
#include <stdint.h>
#include "uart.h"
#include "util.h"

static inline void setup_gpio(int gpio_mask) {

    mmio_write(GPPUD, 0x00000000);
    delay(150);

    mmio_write(GPPUDCLK0, gpio_mask);
    delay(150);

    // setup GPIO 14 et 15
    mmio_write(GPPUDCLK0, 0x00000000);
}

void uart0_init() {

    // Disable UART0.
    mmio_write(UART0_DR, 0x00000000);

    setup_gpio((1 << 14) | (1 << 15));

    // Setup baud
    mmio_write(UART0_IBRD, 2);
    mmio_write(UART0_FBRD, 0xB);

    // Enable FIFO & 8 bit data transmission (1 stop bit, no parity).
    mmio_write(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

    mmio_write(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}

void uart_putc(unsigned char c)
{
    // Wait for UART to become ready to transmit.
    while ( mmio_read(UART0_FR) & (1 << 5) ) { }
    mmio_write(UART0_DR, c);
}

unsigned char uart_getc()
{
    // Wait for UART to have received something.
    while ( mmio_read(UART0_FR) & (1 << 4) ) { }
    return mmio_read(UART0_DR);
}
