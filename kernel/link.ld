ENTRY(_boot_cores);

SECTIONS
{
    . = 0x3ee00000;

    .text :
    {
        KEEP(*(.text.boot)) *(.text .text.*)
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}