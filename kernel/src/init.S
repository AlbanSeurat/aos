.section ".text.boot"

.global _init

_init:
1:  wfe
    b       1b