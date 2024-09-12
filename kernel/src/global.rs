use mmio::{BCMDeviceMemory, Uart};
use crate::memory;
use qemu_exit::QEMUExit;
use crate::scheduler::Scheduler;
use core::time::Duration;
use mmio::timer::{PhysicalTimer, SystemTimer};
use core::fmt::Debug;
use usb::bus::UsbBus;

pub const BCMDEVICES: BCMDeviceMemory = BCMDeviceMemory::new(memory::map::virt::peripheral::START);
pub const IRQ: mmio::IRQ = mmio::IRQ::new(memory::map::virt::IRQ_BASE);
pub const UART: Uart = mmio::Uart::new(memory::map::virt::UART_BASE);
pub const PTIMER: PhysicalTimer = PhysicalTimer::new(Duration::from_millis(100));
pub const STIMER: SystemTimer = SystemTimer::new(memory::map::virt::SYS_TIMER_BASE);
//pub const USB: USB = mmio::USB::new(memory::map::virt::USB_BASE, &STIMER);
pub const USB: UsbBus = UsbBus::new(memory::map::virt::USB_BASE,  &STIMER);
pub static mut SCHEDULER: Scheduler = Scheduler::new();

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}
