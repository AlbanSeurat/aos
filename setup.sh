curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
rustup component add rust-src llvm-tools-preview --toolchain=nightly
cargo install cargo-xbuild
rustup target add aarch64-unknown-none
