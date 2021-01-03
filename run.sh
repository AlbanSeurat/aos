#!/bin/zsh

size=$(stat -f %z kernel-high.img)
bsize=$(printf "0: %.4x" "$size" | xxd -r -g0)
kernel=$(<kernel-high.img)
echo "$bsize$kernel" | ../qemu/build/qemu-system-aarch64 -m 1024 -M raspi3 -serial stdio -semihosting -kernel kernel8.img $@

