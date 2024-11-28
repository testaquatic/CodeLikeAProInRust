#![feature(allocator_api)]

use std::{alloc::Allocator, ptr::NonNull};

use libc::{free, malloc};

pub struct BasicAllocator;

unsafe impl Allocator for BasicAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            // C 라이브러리의 malloc()을 호출한다.
            let ptr = malloc(layout.size() as libc::size_t);
            // 원시 C 포인터를 러스트 슬라이스로 변환한다.
            let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) {
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}

#[test]
fn test_basic_allocator() {
    let mut custom_alloc_vec = Vec::with_capacity_in(10, BasicAllocator);
    (0..10).for_each(|i| custom_alloc_vec.push(i));

    let test_vec = (0..10).collect::<Vec<_>>();

    assert_eq!(custom_alloc_vec, test_vec);
}
