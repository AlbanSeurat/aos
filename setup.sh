curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild cargo-binutils
rustup component add clippy-preview --toolchain=nightly
