#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(duration_constants)]
#![feature(llvm_asm)]

#[macro_use] extern crate mmio;
use cortex_a::asm;
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

/// Entrypoint of the program
#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn _main() -> () {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    mmio::SCREEN.appender(SysCall { }.into());

    println!("show a message using SVC call");

    let syscall = SysCall { };
    syscall.sleep(1);

    println!("show a second message after one second");

    syscall.halt();

}
