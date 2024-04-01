mod hcd;


pub enum USBError {

}

pub struct USB {
    base_addr: usize,
}

impl USB {
    pub const fn new(base_addr: usize) -> USB {
        USB { base_addr }
    }

    pub fn init(&self) -> Result<(), USBError> {

        Ok(())
    }
}