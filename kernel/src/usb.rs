use alloc::boxed::Box;
use crate::global::USB;
use core::ops::Deref;
use mmio::Mbox;

pub fn setup_usb(v_mbox: &mut Mbox) {

    /*let mut device = Box::new_in(UsbDevice::new(), DmaAllocator);

    USB.init(v_mbox).expect("Issue with USB.init");
    USB.start().expect("ISSUE with USB.start");

    let request = Box::new_in(UsbDeviceRequest {
        requestType: 0x80,
        requestName: 6,
        value: 1 << 8,
        index: 0,
        length: 8
    }, DmaAllocator);

    USB.send_request(device.as_mut(), request.as_ref()).expect("ISSUE with USB.attachRoot");

     */


    USB.init().expect("USB.init failed");

    debugln!("USB starting...");
}



