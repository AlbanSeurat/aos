ENTRY(_boot_cores);

SECTIONS
{
    . = 0x80000;
    .text :
    {
        KEEP(*(.text.boot)) *(.text .text.*)
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4096); /* Fill up to 4KiB */
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
}