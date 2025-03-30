use std::{
    alloc::{self, Allocator, Global},
    ptr::NonNull,
};

pub struct PassThruAllocator;

unsafe impl Allocator for PassThruAllocator {
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe { Global.deallocate(ptr, layout) }
    }
}

mod tests {

    #[test]
    fn test_pass_thru_allocator() {
        let mut v = Vec::new_in(crate::PassThruAllocator);
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
