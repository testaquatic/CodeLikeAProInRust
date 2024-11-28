#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator},
    ffi::c_void,
    ptr,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref PAGESIZE: usize = {
        #[cfg(unix)]
        {
            use libc::{sysconf, _SC_PAGESIZE};

            unsafe { sysconf(_SC_PAGESIZE) as usize }
        }
        #[cfg(windows)]
        {
            use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};

            let mut si = SYSTEM_INFO::default();
            unsafe { GetSystemInfo(&mut si) };
            si.dwPageSize as usize
        }
    };
}
/// | fore_protected_region(mprotect_noaccess) | data(mprotect_readwrite) | aft_protected_region(mprotect_noaccess) |
pub struct PageAlignedAllocator;

unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, AllocError> {
        let pagesize = *PAGESIZE;
        let size = _page_round(layout.size(), pagesize) + 2 * pagesize;
        #[cfg(unix)]
        use libc::posix_memalign;

        let out = {
            let mut out = ptr::null_mut();
            let ret = unsafe { posix_memalign(&mut out, pagesize, size) };
            if ret != 0 {
                return Err(AllocError);
            }
            out
        };
        #[cfg(windows)]
        let out = {
            use winapi::um::memoryapi::VirtualAlloc;
            use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

            unsafe {
                VirtualAlloc(
                    ptr::null_mut(),
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE,
                )
            }
        };
        let fore_protected_region =
            unsafe { std::slice::from_raw_parts_mut(out as *mut u8, pagesize) };
        mprotect_noaccess(fore_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}, in allocator", err))
            .ok();
        let aft_protected_region_offset = pagesize + _page_round(layout.size(), pagesize);
        let aft_protected_region = unsafe {
            std::slice::from_raw_parts_mut(
                out.add(aft_protected_region_offset) as *mut u8,
                pagesize,
            )
        };
        mprotect_noaccess(aft_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?} in allocator", err))
            .ok();
        let slice =
            unsafe { std::slice::from_raw_parts_mut(out.add(pagesize) as *mut u8, layout.size()) };
        mprotect_readwrite(slice)
            .map_err(|err| eprintln!("mprotect error = {:?}, in allocator", err))
            .ok();

        unsafe { Ok(ptr::NonNull::new_unchecked(slice)) }
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: std::alloc::Layout) {
        let pagesize = *PAGESIZE;
        let ptr = ptr.as_ptr().offset(-(pagesize as isize));
        let fore_protected_region = std::slice::from_raw_parts_mut(ptr, pagesize);
        mprotect_readwrite(fore_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}", err))
            .ok();
        let aft_protected_region_offset = pagesize + _page_round(layout.size(), pagesize);
        let aft_protected_region =
            std::slice::from_raw_parts_mut(ptr.add(aft_protected_region_offset), pagesize);
        mprotect_readwrite(aft_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}", err))
            .ok();

        #[cfg(unix)]
        {
            libc::free(ptr as *mut libc::c_void);
        }
        #[cfg(windows)]
        {
            use winapi::shared::minwindef::LPVOID;
            use winapi::um::memoryapi::VirtualFree;
            use winapi::um::winnt::MEM_RELEASE;
            VirtualFree(ptr as LPVOID, 0, MEM_RELEASE);
        }
    }
}

fn _page_round(size: usize, pagesize: usize) -> usize {
    // pagesize와 size가 일치하면 블록이 더 할당될 것 같다?
    size + (pagesize - size % pagesize)
}

fn mprotect_noaccess(data: &[u8]) -> Result<(), std::io::Error> {
    if data.is_empty() {
        return Ok(());
    }
    #[cfg(unix)]
    {
        let ret = unsafe {
            libc::mprotect(
                data.as_ptr() as *mut c_void,
                data.len() - 1,
                libc::PROT_NONE,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
    #[cfg(windows)]
    {
        use winapi::shared::minwindef::{DWORD, LPVOID};
        use winapi::um::memoryapi::VirtualProtect;
        use winapi::um::winnt::PAGE_NOACCESS;

        let mut old: DWORD = 0;

        let res = unsafe {
            VirtualProtect(
                data.as_ptr() as LPVOID,
                data.len() - 1,
                PAGE_NOACCESS,
                &mut old,
            )
        };
        match res {
            1 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
}

fn mprotect_readwrite(data: &[u8]) -> Result<(), std::io::Error> {
    if data.is_empty() {
        return Ok(());
    }

    #[cfg(unix)]
    {
        let ret = unsafe {
            libc::mprotect(
                data.as_ptr() as *mut c_void,
                data.len() - 1,
                libc::PROT_READ | libc::PROT_WRITE,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }

    #[cfg(windows)]
    {
        use winapi::shared::minwindef::{DWORD, LPVOID};
        use winapi::um::memoryapi::VirtualProtect;
        use winapi::um::winnt::PAGE_READWRITE;

        let mut old: DWORD = 0;

        let res = unsafe {
            VirtualProtect(
                data.as_ptr() as LPVOID,
                data.len() - 1,
                PAGE_READWRITE,
                &mut old,
            )
        };
        match res {
            1 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
}

#[test]
fn test_custom_allocator() {
    let mut custom_alloc_vec = Vec::with_capacity_in(10, PageAlignedAllocator);
    (0..10).for_each(|i| custom_alloc_vec.push(i));
    let default_vec = (0..10).collect::<Vec<_>>();

    assert_eq!(custom_alloc_vec, default_vec);
}
