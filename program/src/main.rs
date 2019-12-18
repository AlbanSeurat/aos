#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]

#[macro_use] extern crate mmio;
use cortex_a::asm;
use mmio::syscall::SysCall;

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    asm::wfe();
    loop {}
}


/// Entrypoint of the kernel.
///
/// No need to park other processor it has been done by boot.c
#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn _main() -> () {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let syscall = SysCall { };
    mmio::LOGGER.appender(syscall.into());

    debugln!("show a message using SCV call");

    loop {
        asm::wfe();
    }
}
