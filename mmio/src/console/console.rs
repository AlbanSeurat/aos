use console_traits::{UnicodeConsole, Position, BaseConsole, Col, Row, ControlCharMode, EscapeCharMode};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use crate::console::fb::FrameBuffer;
use crate::mbox;
use crate::io::{Writer, IoResult};

pub struct FrameBufferConsole {
    lfb: FrameBuffer,
    pos: Position,
    inc: u32,
    v_mbox: mbox::Mbox<'static>,
}

impl FrameBufferConsole {
    pub fn new(v_mbox: mbox::Mbox<'static>, baseaddr: usize) -> FrameBufferConsole {
        let mut boxx = v_mbox;
        FrameBufferConsole {
            lfb : FrameBuffer::new(&mut boxx, baseaddr).unwrap(),
            pos : Position::origin(),
            inc : 0,
            v_mbox : boxx
        }
    }
}

impl UnicodeConsole for FrameBufferConsole {

    fn write_char_at(&mut self, ch: char, pos: Position) -> Result<(), Self::Error> {

        let x  = pos.col.0 as u32 * 8;
        let y =  (pos.row.0 as u32 + self.inc) * 8;

        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (yy, g) in glyph.iter().enumerate() {
                for xx  in 0..8 {
                    match *g & 1 << xx {
                        0 => self.lfb.print_pixel(x + xx as u32, y + yy as u32, 0x00),
                        _ => self.lfb.print_pixel(x + xx as u32, y + yy as u32, 0xFFFFFFFF),
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_escape(&mut self, _escaped_char: char) -> bool {
        unimplemented!()
    }
}

impl BaseConsole for FrameBufferConsole {
    type Error = &'static str;

    fn get_width(&self) -> Col {
        return Col((self.lfb.width / 8) as u8);
    }

    fn get_height(&self) -> Row {
        return Row((self.lfb.height / 8) as u8);
    }

    fn set_col(&mut self, _col: Col) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_row(&mut self, _row: Row) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_pos(&mut self, pos: Position) -> Result<(), Self::Error> {
        self.pos = pos;
        Ok(())
    }

    fn get_pos(&self) -> Position {
        return self.pos;
    }

    fn set_control_char_mode(&mut self, _mode: ControlCharMode) {
        unimplemented!()
    }

    fn get_control_char_mode(&self) -> ControlCharMode {
        return ControlCharMode::Interpret;
    }

    fn set_escape_char_mode(&mut self, _mode: EscapeCharMode) {
        unimplemented!()
    }

    fn get_escape_char_mode(&self) -> EscapeCharMode {
        return EscapeCharMode::Waiting;
    }

    fn scroll_screen(&mut self) -> Result<(), Self::Error> {
        self.lfb.scroll();
        Ok(())
    }
}

impl Writer for FrameBufferConsole {

    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.write_string(core::str::from_utf8(buf).expect("malformated utf8 string"));
        Ok(buf.len())
    }
}