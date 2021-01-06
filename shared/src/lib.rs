#![no_std]
#![feature(global_asm)]
#![feature(const_generics)]
#![feature(llvm_asm)]

extern crate mmio;
pub mod exceptions;
pub mod memory;