project (AoS-Boot)

enable_language(C ASM)

SET(CMAKE_SYSTEM_NAME aos)
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_C_COMPILER "aarch64-none-elf-gcc")
set(CMAKE_ASM_COMPILER "aarch64-none-elf-as")
set(CMAKE_C_LINK_EXECUTABLE "aarch64-none-elf-gcc -T linker.ld -ffreestanding -O2 -nostdlib -lgcc <OBJECTS> -o <TARGET>")

set(CMAKE_ASM_FLAGS "-EL")
set(CMAKE_C_FLAGS "-ffreestanding -o2 -march=armv8-a+nofp -Wall -Wextra")

include_directories(includes)

add_executable(boot.elf src/boot.s src/boot.c src/io/uart.c src/util/kprintf.c)

add_custom_target(boot.img ALL
        DEPENDS boot.elf
        COMMAND aarch64-none-elf-objcopy out/boot.elf -O binary out/boot.img)

set_target_properties( boot.elf
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/lib"
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/lib"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/out"
        )
