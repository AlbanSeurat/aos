# AoS

Small playground bare-metal programming for raspberry pi 3+ (my own)

I have been inspired by https://github.com/bztsrc/raspi3-tutorial and https://github.com/rust-embedded/rust-raspi3-OS-tutorials


## setup env`(macOS)

```./setup.sh```

## compile and run 

* to compile rust programs

```make```

* to run the kernel and programs

setup the tty to simulate the serial 

```socat pty,link=cu.serial-master,raw,echo=0 pty,link=cu.serial-slave,raw,echo=0```

prepare the sender 

```python3.9 load.py cu.serial-slave```

run arguments

```./run.sh```

#### show instructions and interruption

```./run.sh -d in_asm,int ```


## Run on real hardware 

* wire the cable et setup the link 

```python3 load.py `ls /dev/cu.usb*` ```

* launch raspi (python will show the debug output)

## copy on SD card and run on Raspi

Files to be found here : 
https://github.com/raspberrypi/firmware/tree/master/boot

 - bootcode.bin
 - start.elf
 - fixup.dat

Schematics with my USB-to-serial 


## Troubleshooting 

#### Exception Handling 

Reference : SysReg_xml_v85A-2019-06.pdf (page 418)

ESR_ELx : Register holding exception information : 

EC : Exception Class 


#### Disassembling 

```aarch64-none-elf-objdump -b binary -maarch64 -D program.img```

