#!/bin/bash

#qemu-system-aarch64 -d int,in_asm -M raspi3 -serial stdio -kernel out/kernel8.img
qemu-system-aarch64 -d int -m 1024 -M raspi3 -serial stdio -kernel boot/out/boot.img $@ 


