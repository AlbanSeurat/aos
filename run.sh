#!/bin/bash

#qemu-system-aarch64 -d int,in_asm -M raspi3 -serial stdio -kernel out/kernel8.img
qemu-system-aarch64 -d int -m 128 -M raspi3 -serial stdio -kernel out/kernel8.img $@


