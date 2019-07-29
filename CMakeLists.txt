cmake_minimum_required (VERSION 3.6)
project (AoS)

enable_language(C ASM)

SET(CMAKE_SYSTEM_NAME aos)
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_C_COMPILER "aarch64-none-elf-gcc")
set(CMAKE_ASM_COMPILER "aarch64-none-elf-as")
set(CMAKE_C_LINK_EXECUTABLE "aarch64-none-elf-gcc -T linker.ld -ffreestanding -O2 -nostdlib -lgcc <OBJECTS> -o <TARGET>")

set(CMAKE_ASM_FLAGS "-EL")
set(CMAKE_C_FLAGS "-ffreestanding -o2 -march=armv8-a+nofp -Wall -Wextra")

include_directories(boot/includes)

add_executable(kernel.elf boot/src/boot.s boot/src/kernel.c boot/src/io/uart.c boot/src/mmu.c
        boot/src/util/kprintf.c boot/src/io/exc.c boot/src/io/exc.s boot/src/io/timer.c)

add_custom_target(kernel8.img ALL
        DEPENDS kernel.elf kernel8.img
        COMMAND aarch64-none-elf-objcopy out/kernel.elf -O binary out/kernel8.img)

set_target_properties( kernel.elf
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/out"
        )
