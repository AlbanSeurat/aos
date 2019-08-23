#![no_std]
#![no_main]

use cortex_a::asm;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    asm::wfe();
    loop {}
}

#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    asm::wfe();

    loop {}
}