cmake_minimum_required (VERSION 3.6)
project (AoS)

enable_language(C ASM)

SET(CMAKE_SYSTEM_NAME aos)
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_C_COMPILER "aarch64-none-elf-gcc")
set(CMAKE_ASM_COMPILER "aarch64-none-elf-as")
set(CMAKE_C_LINK_EXECUTABLE "aarch64-none-elf-gcc -T linker.ld -ffreestanding -O2 -nostdlib -lgcc <OBJECTS> -o <TARGET>")

set(CMAKE_ASM_FLAGS "-EL")
set(CMAKE_C_FLAGS "-ffreestanding -O2 -Wall -Wextra")

include_directories(includes)

add_executable(kernel.elf src/boot.s src/kernel.c src/uart.c src/printf.c src/exc.c src/exc.s)

add_custom_target(kernel8.img ALL
        DEPENDS kernel.elf kernel8.img
        COMMAND aarch64-none-elf-objcopy out/kernel.elf -O binary out/kernel8.img)

set_target_properties( kernel.elf
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/out"
        )
