#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(duration_constants)]

#[macro_use] extern crate mmio;
use mmio::syscall::SysCall;
use qemu_exit::QEMUExit;

use aarch64_cpu::registers::{Readable, SP};

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}

/// Entrypoint of the program
#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn _main() -> () {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    mmio::SCREEN.appender(SysCall { }.into());

    println!("show a message using SVC call");

    let mut count:u128 = 0;
    loop {
        if count % 10000 == 0 {
            println!("current stack pointer {:x}", SP.get());
            println!("show string from time to time {}", count);
        }
        count = count + 1;
    }

}
