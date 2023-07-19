use core::alloc::{Allocator, AllocError, Layout, GlobalAlloc};
use core::ptr::{NonNull};
use core::cell::RefCell;
use core::{mem, slice};
use linked_list_allocator::{Heap, LockedHeap};
use crate::DMA;

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

pub struct DmaAllocator;

unsafe impl Allocator for DmaAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let allocated = unsafe { core::slice::from_raw_parts_mut(DMA.alloc(layout), layout.size()) };
        Ok(NonNull::new(allocated).expect("Null Allocation"))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        DMA.dealloc(ptr.as_ptr(), layout)
    }
}