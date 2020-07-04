all: kernel
kernel: elf
	rust-objcopy --strip-all -O binary ../target/aarch64-unknown-none/release/aos-kernel ../kernel-high.img
elf:
	cargo xrustc --target aarch64-unknown-none --release
clean:
	cargo clean