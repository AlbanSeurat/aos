project (AoS-Kernel)

file(GLOB_RECURSE RUST_SOURCES "*.rs")


set(LIB_FILE "target/aarch64-unknown-none/debug/aos-kernel")

add_custom_command(
        OUTPUT ${LIB_FILE}
        COMMAND xargo ARGS build --target aarch64-unknown-none
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS ${RUST_SOURCES}
        COMMENT "running cargo")

add_custom_target(kernel8.img ALL
        DEPENDS ${LIB_FILE}
        COMMAND xargo objcopy --target aarch64-unknown-none -- --strip-all -O binary ${LIB_FILE} kernel8.img)