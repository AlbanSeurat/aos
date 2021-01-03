use byteorder::{ByteOrder, BigEndian};

pub enum IOError {
    UnknownError,
}

pub type IoResult<T> = ::core::result::Result<T, IOError>;

pub trait Writer {
    fn puts(&mut self, _string: &str) -> IoResult<usize>;
}

pub trait Reader {

    fn read_char(&mut self) -> IoResult<u8>;

    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize>;

    fn read_short(&mut self) -> IoResult<u16> {
        let mut short = 0u16.to_be_bytes();
        self.read(&mut short)?;
        Ok(BigEndian::read_u16(&short))
    }
}