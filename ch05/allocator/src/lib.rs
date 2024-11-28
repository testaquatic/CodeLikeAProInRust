#![feature(allocator_api)]

use std::alloc::{Allocator, Global};

pub struct PassThruAllocator;

unsafe impl Allocator for PassThruAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        Global.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        Global.deallocate(ptr, layout)
    }
}
