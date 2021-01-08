use mmio::{BCMDeviceMemory, Uart, DWHCI};
use crate::memory;
use qemu_exit::QEMUExit;

pub const BCMDEVICES: BCMDeviceMemory = BCMDeviceMemory::new(memory::map::virt::peripheral::START);
pub const DWHCI: DWHCI = mmio::DWHCI::new(memory::map::virt::USB_BASE);
pub const IRQ: mmio::IRQ = mmio::IRQ::new(memory::map::virt::IRQ_BASE);

pub const UART: Uart = mmio::Uart::new(memory::map::virt::UART_BASE);

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}
