ENTRY(_boot_cores);

SECTIONS
{
    /* . = 0x3ae00000; */
    . = 0xFFFFFFFFFae00000;
    __ro_start = .;
    .text :
    {
        KEEP(*(.text.boot)) *(.text .text.*)
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4096); /* Fill up to 4KiB */
    __ro_end = .;

    .data :
    {
        *(.data .data.*)
    }

    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }

    /*
    align 4096;
    . = 0xFFFFFFFFC0000000;
    .text :
    {
        KEEP(*(.text.upper))
    }
    */

}