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
	.balign 0x80
	b	hang // synchronous
	.balign 0x80
	b	hang // IRQ
	.balign 0x80
	b	hang // FIQ
	.balign 0x80
	b	hang // SError


