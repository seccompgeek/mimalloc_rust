#![no_std]
//! This is a drop-in global allocator wrapper around mimalloc for MetaSafe
extern crate mimalloc_sys as ffi;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr;

#[global_allocator]
pub static ALLOC: MetaSafeAlloc = MetaSafeAlloc;

/// Drop-in mimalloc global allocator
/// designed for MetaSafe
pub struct MetaSafeAlloc;

unsafe impl GlobalAlloc for MetaSafeAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::mi_malloc_aligned(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = ffi::mi_malloc_aligned(layout.size(), layout.align());
        if !ptr.is_null() {
            ptr::write_bytes(ptr as *mut u8, 0, layout.size())
        }
        ptr as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ffi::mi_free_aligned(ptr as *mut c_void, layout.align())
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ffi::mi_realloc_aligned(ptr as *mut c_void, new_size, layout.align()) as *mut u8
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
