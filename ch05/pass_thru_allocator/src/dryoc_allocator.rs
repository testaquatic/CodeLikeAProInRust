use std::{
    alloc::{self, Allocator},
    ptr::{self, NonNull},
    sync::LazyLock,
};

#[cfg(unix)]
use libc::{_SC_PAGESIZE, PROT_READ, PROT_WRITE, c_void, mprotect, sysconf};

#[cfg(unix)]
static PAGESIZE: LazyLock<usize> = LazyLock::new(|| {
    #[cfg(unix)]
    {
        unsafe { sysconf(_SC_PAGESIZE) as usize }
    }
});

#[cfg(unix)]
fn mprotect_readwrite(data: &[u8]) -> Result<(), std::io::Error> {
    if data.is_empty() {
        return Ok(());
    }

    let ret = unsafe {
        mprotect(
            data.as_ptr() as *mut c_void,
            data.len() - 1,
            PROT_READ | PROT_WRITE,
        )
    };
    match ret {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error()),
    }
}

#[cfg(unix)]
fn mprotect_noaccess(data: &[u8]) -> Result<(), std::io::Error> {
    use libc::PROT_NONE;

    if data.is_empty() {
        return Ok(());
    }

    let ret = unsafe { mprotect(data.as_ptr() as *mut c_void, data.len() - 1, PROT_NONE) };
    match ret {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error()),
    }
}

fn page_round(size: usize, page_size: usize) -> usize {
    size + (page_size - size % page_size)
}

pub struct DryocAllocator;

unsafe impl Allocator for DryocAllocator {
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        let pagesize = *PAGESIZE;
        let size = page_round(layout.size(), pagesize) + 2 * pagesize;
        let mut out = ptr::null_mut();
        let ret = unsafe { libc::posix_memalign(&mut out, pagesize, size) };
        if ret != 0 {
            return Err(alloc::AllocError);
        }

        let fore_protected_region =
            unsafe { std::slice::from_raw_parts_mut(out as *mut u8, pagesize) };
        mprotect_noaccess(fore_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}, in allocator", err))
            .ok();
        let aft_protected_region_offset = pagesize + page_round(layout.size(), pagesize);
        let art_protected_region = unsafe {
            std::slice::from_raw_parts_mut(
                out.add(aft_protected_region_offset) as *mut u8,
                pagesize,
            )
        };
        mprotect_noaccess(art_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}, in allocator", err))
            .ok();
        let slice =
            unsafe { std::slice::from_raw_parts_mut(out.add(pagesize) as *mut u8, layout.size()) };
        mprotect_readwrite(slice)
            .map_err(|err| eprintln!("mprotect error = {:?}, in allocator", err))
            .ok();

        unsafe { Ok(NonNull::new_unchecked(slice)) }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: alloc::Layout) {
        let pagesize = *PAGESIZE;
        let ptr = unsafe { ptr.as_ptr().offset(-(pagesize as isize)) };
        let for_protected_region = unsafe { std::slice::from_raw_parts_mut(ptr, pagesize) };

        mprotect_readwrite(for_protected_region)
            .map_err(|err| {
                eprintln!("mprotect error = {:?}, in deallocator", err);
            })
            .ok();
        let aft_protected_region_offset = pagesize + page_round(_layout.size(), pagesize);
        let aft_protected_region = unsafe {
            std::slice::from_raw_parts_mut(ptr.add(aft_protected_region_offset), pagesize)
        };
        mprotect_readwrite(aft_protected_region)
            .map_err(|err| {
                eprintln!("mprotect error = {:?}, in deallocator", err);
            })
            .ok();

        unsafe { libc::free(ptr as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dryoc_allocator() {
        let mut v = Vec::new_in(crate::DryocAllocator);
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
