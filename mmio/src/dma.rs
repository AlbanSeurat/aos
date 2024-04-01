use core::alloc::{AllocError, Layout, GlobalAlloc};
use core::{mem, slice};
use linked_list_allocator::{LockedHeap};

pub trait SliceAllocator {
    fn alloc_slice_zeroed<'a, T>(
        &self,
        count_of_items: usize,
        alignment: usize,
    ) -> Result<&'a mut [T], AllocError>;
}


impl SliceAllocator for LockedHeap {
    /// Allocate a zeroed slice
    fn alloc_slice_zeroed<'a, T>(
        &self,
        count_of_items: usize,
        alignment: usize,
    ) -> Result<&'a mut [T], AllocError> {
        let size_in_byte = count_of_items * mem::size_of::<T>();
        let l = Layout::from_size_align(size_in_byte, alignment).map_err(|_| AllocError {})?;

        Ok(unsafe { slice::from_raw_parts_mut(self.alloc_zeroed(l) as *mut T, count_of_items) })
    }
}
