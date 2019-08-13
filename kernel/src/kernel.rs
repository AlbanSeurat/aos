use cortex_a::asm;

pub(crate) fn main() -> ! {
    asm::wfe();
    loop {}
}
