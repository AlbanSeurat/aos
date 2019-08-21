use crate::kernel::devices::hw::FrameBuffer;
use console_traits::{UnicodeConsole, Position, BaseConsole, Col, Row, ControlCharMode, EscapeCharMode};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use core::fmt;

pub struct Console {
    lfb: FrameBuffer,
    pos: Position,
}

impl Console {
    pub fn new(fb: FrameBuffer) -> Console {
        Console {
            lfb : fb,
            pos : Position::origin(),
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s).unwrap();
        Ok(())
    }
}

impl UnicodeConsole for Console {

    fn write_char_at(&mut self, ch: char, pos: Position) -> Result<(), Self::Error> {

        let x  = pos.col.0 as u32 * 8;
        let y = pos.row.0 as u32 * 8;

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

    fn handle_escape(&mut self, escaped_char: char) -> bool {
        unimplemented!()
    }
}

impl BaseConsole for Console {
    type Error = ();

    fn get_width(&self) -> Col {
        return Col(128)
    }

    fn get_height(&self) -> Row {
        return Row(96)
    }

    fn set_col(&mut self, col: Col) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_row(&mut self, row: Row) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_pos(&mut self, pos: Position) -> Result<(), Self::Error> {
        self.pos = pos;
        Ok(())
    }

    fn get_pos(&self) -> Position {
        return self.pos;
    }

    fn set_control_char_mode(&mut self, mode: ControlCharMode) {
        unimplemented!()
    }

    fn get_control_char_mode(&self) -> ControlCharMode {
        return ControlCharMode::Interpret;
    }

    fn set_escape_char_mode(&mut self, mode: EscapeCharMode) {
        unimplemented!()
    }

    fn get_escape_char_mode(&self) -> EscapeCharMode {
        return EscapeCharMode::Waiting;
    }

    fn scroll_screen(&mut self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
