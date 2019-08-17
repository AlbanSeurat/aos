project (AoS-Kernel)

file(GLOB RUST_SOURCES "*.rs")

set(LIB_FILE "target/aarch64-unknown-none/release/aos-kernel")

add_custom_command(
        OUTPUT ${LIB_FILE}
        COMMAND cargo ARGS xrustc --target aarch64-unknown-none --release
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS ${RUST_SOURCES}
        COMMENT "running cargo")

add_custom_target(kernel8.img ALL
        DEPENDS ${LIB_FILE}
        COMMAND cargo objcopy -- --strip-all -O binary ${LIB_FILE} kernel8.img)

