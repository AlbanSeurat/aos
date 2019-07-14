#!/bin/bash

#qemu-system-aarch64 -d int,in_asm -M raspi3 -serial stdio -kernel out/kernel8.img
qemu-system-aarch64 -d int -M raspi3 -serial stdio -kernel out/kernel8.img


