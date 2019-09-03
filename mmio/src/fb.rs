use crate::kernel::devices::hw::mbox;
use crate::kernel::memory;
use core::sync::atomic::{compiler_fence, Ordering};
use core::ptr;

#[derive(Debug)]
pub struct FrameBuffer {
    width : u32,
    height: u32,
    pitch: u32,
    base_pointer: usize,
}
const BPP : u32 = 32;

impl FrameBuffer {
    pub fn new(v_mbox: &mut mbox::Mbox) -> FrameBuffer {

        const REQUEST_SIZE : u32 = 35 * 4;

        v_mbox.buffer[0] = REQUEST_SIZE;
        v_mbox.buffer[1] = mbox::REQUEST;

        v_mbox.buffer[2] = mbox::tag::SET_SCREEN_PHY_RES;
        v_mbox.buffer[3] = 8;
        v_mbox.buffer[4] = 8;
        v_mbox.buffer[5] = 1024;         //FrameBufferInfo.width
        v_mbox.buffer[6] = 768;          //FrameBufferInfo.height

        v_mbox.buffer[7] = mbox::tag::SET_SCREEN_VIRT_RES;
        v_mbox.buffer[8] = 8;
        v_mbox.buffer[9] = 8;
        v_mbox.buffer[10] = 1024;         //FrameBufferInfo.virtual_width
        v_mbox.buffer[11] = 768;          //FrameBufferInfo.virtual_height

        v_mbox.buffer[12] = mbox::tag::SET_SCREEN_VIRT_OFF;
        v_mbox.buffer[13] = 8;
        v_mbox.buffer[14] = 8;
        v_mbox.buffer[15] = 0;           //FrameBufferInfo.x_offset
        v_mbox.buffer[16] = 0;           //FrameBufferInfo.y.offset

        v_mbox.buffer[17] = mbox::tag::SET_SCREEN_DEPTH;
        v_mbox.buffer[18] = 4;
        v_mbox.buffer[19] = 4;
        v_mbox.buffer[20] = 32;          //FrameBufferInfo.depth

        v_mbox.buffer[21] = mbox::tag::SET_SCREEN_ORDER;
        v_mbox.buffer[22] = 4;
        v_mbox.buffer[23] = 4;
        v_mbox.buffer[24] = 1;           //RGB, not BGR preferably

        v_mbox.buffer[25] = mbox::tag::GET_SCREEN_FRAME_BUFFER; //get framebuffer, gets alignment on request
        v_mbox.buffer[26] = 8;
        v_mbox.buffer[27] = 8;
        v_mbox.buffer[28] = 4096;        //FrameBufferInfo.pointer
        v_mbox.buffer[29] = 0;           //FrameBufferInfo.size

        v_mbox.buffer[30] = mbox::tag::GET_PITCH; //get pitch
        v_mbox.buffer[31] = 4;
        v_mbox.buffer[32] = 4;
        v_mbox.buffer[33] = 0;           //FrameBufferInfo.pitch

        v_mbox.buffer[34] = mbox::tag::LAST;

        compiler_fence(Ordering::Release);

        if v_mbox.call(mbox::channel::PROP).is_ok() && v_mbox.buffer[20] == 32 && v_mbox.buffer[28] != 0 {
            return FrameBuffer {
                width: v_mbox.buffer[5],
                height: v_mbox.buffer[6],
                pitch: v_mbox.buffer[33],
                base_pointer: memory::map::virt::START + v_mbox.buffer[28] as usize,
            }
        } else {
            panic!("Error setting up screen");
        };

    }

    pub fn print_pixel(&self, x : u32, y: u32, pixel: u32) {
        let pixel_offset : u32 = ( x * ( BPP >> 3 ) ) + ( y * self.pitch );
        unsafe {
            ptr::write((self.base_pointer + pixel_offset as usize) as *mut u32, pixel);
        }
    }
}
