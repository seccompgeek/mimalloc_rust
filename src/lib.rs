#![no_std]
//! This is a drop-in global allocator wrapper around mimalloc for MetaSafe
extern crate mimalloc_sys as ffi;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr;

const MIN_ALIGN: usize = 16;

#[global_allocator]
pub static ALLOC: MetaSafeAlloc = MetaSafeAlloc;

/// Drop-in mimalloc global allocator
/// designed for MetaSafe
pub struct MetaSafeAlloc;

unsafe impl GlobalAlloc for MetaSafeAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() < MIN_ALIGN {
            return ffi::mi_malloc_aligned(layout.size(), MIN_ALIGN) as *mut u8;
        }
        ffi::mi_malloc_aligned(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = if layout.align() < MIN_ALIGN {
            ffi::mi_malloc_aligned(layout.size(), MIN_ALIGN)
        } else {
            ffi::mi_malloc_aligned(layout.size(), layout.align())
        };

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
        let align = if layout.align() < MIN_ALIGN { MIN_ALIGN } else {layout.align()};
        ffi::mi_realloc_aligned(ptr as *mut c_void, new_size, align) as *mut u8
    }
}

pub mod libc_compat {
    use super::*;

    #[inline]
    pub unsafe fn malloc(size: usize) -> *mut u8 {
        ffi::mi_malloc(size) as *mut u8
    }

    #[inline]
    pub unsafe fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
        ffi::mi_realloc(ptr as *mut c_void, new_size) as *mut u8
    }

    #[inline]
    pub unsafe fn free(ptr: *mut u8){
        ffi::mi_free(ptr as *mut c_void)
    }

    #[inline]
    pub unsafe fn malloc_usable_size(ptr: *const u8) -> usize {
        ffi::mi_usable_size(ptr as *mut c_void)
    }
}