.section ".text"
.balign 0x800
.global exception_vector
exception_vector:
    b	hang // synchronous
    .balign 0x80
	b	hang // irq
	.balign 0x80
	b	hang // fiq
	.balign 0x80
	b	hang // SError
	.balign 0x80
	b	hang // synchronous
	.balign 0x80
	b	irq // IRQ
	.balign 0x80
	b	hang // FIQ
	.balign 0x80
	b	hang // SError
	.balign 0x80
	b	hang // synchronous
	.balign 0x80
	b	hang // IRQ
	.balign 0x80
	b	hang // FIQ
	.balign 0x80
	b	hang // SError
	.balign 0x80
	b	hang // synchronous
	.balign 0x80
	b	hang // IRQ
	.balign 0x80
	b	hang // FIQ
	.balign 0x80
	b	hang // SError

irq:
    stp   x0,  x1,  [sp, #-16]!
    stp   x2,  x3,  [sp, #-16]!
    stp   x4,  x5,  [sp, #-16]!
    stp   x6,  x7,  [sp, #-16]!
    stp   x8,  x9,  [sp, #-16]!
    stp   x10, x11, [sp, #-16]!
    stp   x12, x13, [sp, #-16]!
    stp   x14, x15, [sp, #-16]!
    stp   x16, x17, [sp, #-16]!
    stp   x18, x19, [sp, #-16]!

    bl    c_irq_handler

    ldp   x18, x19, [sp], #16
    ldp   x16, x17, [sp], #16
    ldp   x14, x15, [sp], #16
    ldp   x12, x13, [sp], #16
    ldp   x10, x11, [sp], #16
    ldp   x8,  x9,  [sp], #16
    ldp   x6,  x7,  [sp], #16
    ldp   x4,  x5,  [sp], #16
    ldp   x2,  x3,  [sp], #16
    ldp   x0,  x1,  [sp], #16
    eret