use mmio::{BCMDeviceMemory, Uart, DWHCI, PhysicalTimer};
use crate::memory;
use qemu_exit::QEMUExit;
use crate::scheduler::Scheduler;
use core::time::Duration;

pub const BCMDEVICES: BCMDeviceMemory = BCMDeviceMemory::new(memory::map::virt::peripheral::START);
pub const DWHCI: DWHCI = mmio::DWHCI::new(memory::map::virt::USB_BASE);
pub const IRQ: mmio::IRQ = mmio::IRQ::new(memory::map::virt::IRQ_BASE);
pub const UART: Uart = mmio::Uart::new(memory::map::virt::UART_BASE);
pub const TIMER: PhysicalTimer = PhysicalTimer::new(Duration::from_millis(100));
pub static mut SCHEDULER: Scheduler = Scheduler::new();

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}

#[alloc_error_handler]
fn foo(layout: core::alloc::Layout) -> ! {
    panic!("Can not allocate {:?}", layout);
}