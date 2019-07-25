.section ".text.boot"

.global _start

_start:
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
1:  wfe
    b       1b
2:  // cpu id == 0

    ldr     x1, =_start

// Timer IRQ can not be received on EL2 => moving to EL1
move_to_el1:
    msr   sp_el1, x1

    // initialize el1 to run in 64 bits
    mov	x0, #(1 << 31)
    orr   x0, x0, #(1 << 1)
    msr	hcr_el2, x0

    // initialize register sctlr_el1 (which control execution of EL1)
    mov	x0, #0x0800
    movk	x0, #0x30d0, lsl #16
    mov	x0, #0x0800
    movk	x0, #0x30d0, lsl #16
    orr    x0, x0, #(0x1 << 2)            // The C bit on (data cache).
    orr    x0, x0, #(0x1 << 12)           // The I bit on (instruction cache)
    msr	sctlr_el1, x0

    // setup exception DAIF properly for EL1
    mov	x0, #0x3c5							// EL1_SP1 | D | A | I | F
	msr	spsr_el2, x0						// Set spsr_el2 with settings
	adr	x0, exit_to_el1						// Address to exit EL2
	msr	elr_el2, x0							// Set elevated return register
	eret
exit_to_el1:

    // set stack before our code
    ldr     x1, =_start
    mov     sp, x1

    // clear bss
    ldr     x1, =__bss_start
    ldr     w2, =__bss_size
3:  cbz     w2, 4f
    str     xzr, [x1], #8
    sub     w2, w2, #1
    cbnz    w2, 3b

    // jump to C code, should not return
4:  bl      main
    // for failsafe, halt this core too
    b       1b
