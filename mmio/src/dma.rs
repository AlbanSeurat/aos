use core::alloc::{Allocator, AllocError, Layout};
use core::ptr::{NonNull};
use core::cell::RefCell;
use core::{mem, slice};

pub struct MemoryRegion {
    next: usize,
    end: usize,
}

impl MemoryRegion {
    pub fn new(start: usize, end: usize) -> Self {
        MemoryRegion {
            next: start,
            end,
        }
    }
}

pub struct DMAMemory {
    region: RefCell<MemoryRegion>
}

impl DMAMemory {
    pub const fn new() -> Self {
        Self { region: RefCell::new(MemoryRegion { next: 0, end: 0 }) }
    }

    pub fn set(&mut self, region: MemoryRegion) {
        self.region = region.into()
    }

    /// Allocate a zeroed slice
    pub fn alloc_slice_zeroed<'a, T>(
        &mut self,
        count_of_items: usize,
        alignment: usize,
    ) -> Result<&'a mut [T], AllocError> {

        let size_in_byte = count_of_items * mem::size_of::<T>();
        let l = Layout::from_size_align(size_in_byte, alignment).map_err(|_| AllocError {})?;

        let ptr = self.allocate_zeroed(l)?.as_ptr();
        Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut T, count_of_items) })
    }
}

unsafe impl Allocator for DMAMemory {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut region = self.region.borrow_mut();

        let start = aligned_addr_unchecked(region.next, layout.align());
        let end = start + layout.size();

        if end <= region.end {
            region.next = end;
            let ptr = NonNull::new(start as *mut u8).ok_or(AllocError)?;
            Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {}
}

#[inline]
fn aligned_addr_unchecked(addr: usize, alignment: usize) -> usize {
    (addr + (alignment - 1)) & !(alignment - 1)
}