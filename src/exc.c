#include <stddef.h>
#include <stdint.h>
#include <printf.h>

extern uint32_t exception_vector[];

void exceptions_init(void) {
    asm volatile("msr vbar_el2, %[base]" :: [base] "r" (exception_vector));
}

void handler_exception() {

}

void hang() {
    printf("panic\n");
    asm volatile("wfe");
}