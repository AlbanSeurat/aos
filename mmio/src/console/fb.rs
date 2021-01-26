use core::sync::atomic::{compiler_fence, Ordering};
use core::{ptr, mem};
use crate::{mbox, DMA};
use crate::{debugln, debug};
use core::alloc::{Allocator, Layout};
use cortex_a::asm;

#[derive(Debug)]
pub struct FrameBufferError {}

pub struct FrameBuffer {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pitch: u32,
    base_pointer: usize,
}

const BPP: u32 = 32;

impl FrameBuffer {
    pub fn new(v_mbox: &mut mbox::Mbox, baseaddr: usize) -> Result<FrameBuffer, FrameBufferError> {

        v_mbox.clear();

        v_mbox.prepare(mbox::tag::SCREEN_PHY_RES, 8, 8, &[1024, 768]);
        v_mbox.prepare(mbox::tag::SCREEN_VIRT_RES, 8, 8, &[1024, 776]);
        v_mbox.prepare(mbox::tag::SET_SCREEN_VIRT_OFF, 8, 8, &[0, 0]);
        v_mbox.prepare(mbox::tag::SET_SCREEN_DEPTH, 4, 4, &[32]);
        v_mbox.prepare(mbox::tag::SET_SCREEN_ORDER, 4, 4, &[1]);
        v_mbox.prepare(mbox::tag::GET_SCREEN_FRAME_BUFFER, 8, 4, &[16, 0]);
        v_mbox.prepare(mbox::tag::GET_PITCH, 4, 4, &[0]);

        let result = v_mbox.request(mbox::channel::PROP);
        if result.is_ok() && v_mbox.dma[20] == 32 && v_mbox.dma[28] != 0 {
            return Ok(FrameBuffer {
                width: v_mbox.dma[5],
                height: v_mbox.dma[6],
                pitch: v_mbox.dma[33],
                base_pointer: (v_mbox.dma[28] & !0xC000_0000 ) as usize,
            });
        } else {
            Err(FrameBufferError {})
        }
    }

    pub fn print_pixel(&self, x: u32, y: u32, pixel: u32) {
        let pixel_offset: u32 = (x * (BPP >> 3)) + (y * self.pitch);
        unsafe {
            ptr::write((self.base_pointer + pixel_offset as usize) as *mut u32, pixel);
        }
    }

    #[inline]
    pub fn scroll(&self) {
        unsafe {
            let nb_of_u64 = self.width * self.height / 2;
            let row_size = self.width as usize * 8usize * mem::size_of::<u32>();
            let last_row = (self.width * self.height) as usize * mem::size_of::<u32>();
            ptr::copy((self.base_pointer + row_size) as *const u64, self.base_pointer as *mut u64, nb_of_u64 as usize);
            ptr::write_bytes((self.base_pointer + last_row) as *mut u64, 0, row_size / mem::size_of::<u64>());
        }
    }
}

