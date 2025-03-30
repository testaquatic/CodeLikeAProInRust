use std::{
    alloc::{self, Allocator},
    ptr::NonNull,
};

pub struct BasicAllocator;

unsafe impl Allocator for BasicAllocator {
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        unsafe {
            let ptr = libc::malloc(layout.size() as libc::size_t);
            let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _: alloc::Layout) {
        unsafe {
            libc::free(ptr.as_ptr() as *mut libc::c_void);
        }
    }
}

mod tests {

    #[test]
    fn test_basic_allocator() {
        let mut vec = Vec::new_in(crate::BasicAllocator);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec, [1, 2, 3]);
    }
}
