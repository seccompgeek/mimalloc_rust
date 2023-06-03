#![no_std]

use core::ffi::c_void;

extern "C" {
    /// allocate size bytes, with the given alignment
    /// returns pointer to the allocated memory
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;

    /// reallocate a previously allocated object, with the given alignment
    /// returns a pointer to the newly allocated object
    pub fn mi_realloc_aligned(p: *mut c_void, new_size: usize, alignment: usize) -> *mut c_void;

    /// frees a given pointer 
    pub fn mi_free_aligned(p: *mut c_void, alignment: usize);

    /// get the usable size of a pointer
    fn mi_usable_size(p: *mut c_void) -> usize;
    
}