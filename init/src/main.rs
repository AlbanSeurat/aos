#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(duration_constants)]

#[macro_use] extern crate mmio;
use aarch64_cpu::asm;
use aarch64_cpu::registers::{CurrentEL, Readable};
use mmio::syscall::SysCall;
use qemu_exit::QEMUExit;

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

/// Entrypoint of the init program
#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn _main() -> () {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    mmio::SCREEN.appender(SysCall {}.into());

    println!("This is the init program, it is the first PID and will fork itself to create other programs");

    let sys_call = SysCall {};
    let mut count:u128 = 0;
    loop {
        if count % 100000000  == 0 {
            println!("init program run at level 0, count {}", count);
            sys_call.sleep(1000);
        }
        count = count + 1;
    }

}
