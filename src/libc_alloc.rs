use core::{
    alloc::{GlobalAlloc, Layout},
    mem::size_of,
    ptr::null_mut,
};

use libc::c_void;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

/// The libc allocator.
struct LibcAlloc;

unsafe impl GlobalAlloc for LibcAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // This is what glibc guarantees, and the smallest reasonable value on x86 and x86_64
        // machines.
        if layout.align() > (size_of::<usize>() * 2) {
            return null_mut();
        }

        libc::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut c_void)
    }
}
