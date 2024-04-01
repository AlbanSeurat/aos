#!/bin/zsh

size=$(stat -f %z kernel-high.img)
bsize=$(printf "0: %.4x" "$size" | xxd -r -g0)
kernel=$(<kernel-high.img)

qemu-system-aarch64 -m 1024 -M raspi3b \
  -chardev serial,path=cu.serial-master,id=char0 \
  -serial chardev:char0 -semihosting -kernel kernel8.img $@
