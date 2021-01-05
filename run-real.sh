#!/bin/zsh

size=$(stat -f %z kernel-high.img)
bsize=$(printf "0: %.4x" "$size" | xxd -r -g0)
kernel=$(<kernel-high.img)
echo "$bsize$kernel" > /dev/cu.usbserial-14210
cat /dev/cu.usbserial-14210
