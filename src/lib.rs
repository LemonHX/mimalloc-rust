#![cfg_attr(feature = "unstable", feature(allocator_api))]
#![cfg_attr(not(test), no_std)]

//! this crate provides the best binding for [mimalloc](https://github.com/microsoft/mimalloc)
//! # Example Usage
//! first add to dependencies
//! ```toml
//! [dependencies]
//! mimalloc-rust = "0.2"
//! ```
//! then set the global allocator
//! ```rust
//! use mimalloc_rust::*;
//!
//! #[global_allocator]
//! static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
//! ```
//! # Allocator API!
//! ```
//! #![feature(allocator_api)]
//! use std::{ffi::c_void, mem::ManuallyDrop};
//!
//! use mimalloc_rust::{
//!     heap::{HeapVisitor, MiMallocHeap},
//!     raw::{
//!         heap::{mi_heap_area_t, mi_heap_delete, mi_heap_new},
//!         types::mi_heap_t,
//!     },
//!     with_heap, GlobalMiMalloc,
//! };
//!
//! #[derive(Debug, Clone)]
//! struct TestHeap {
//!     heap: *mut mi_heap_t,
//! }
//! use std::ops::Deref;
//! impl Deref for TestHeap {
//!     type Target = *mut mi_heap_t;
//!
//!     fn deref(&self) -> &Self::Target {
//!         &self.heap
//!     }
//! }
//!
//! impl TestHeap {
//!     fn new() -> Self {
//!         Self {
//!             heap: unsafe { mi_heap_new() },
//!         }
//!     }
//! }
//!
//! impl Drop for TestHeap {
//!     fn drop(&mut self) {
//!         unsafe { mi_heap_delete(self.heap) }
//!     }
//! }
//!
//! #[test]
//! fn test_allocator_api() {
//!     let allocator = MiMallocHeap::new(TestHeap::new());
//!     let mut b: Vec<u8, &MiMallocHeap<TestHeap>> = Vec::new_in(&allocator);
//!     b.push(1);
//!     b.push(2);
//!     assert_eq!(b[0], 1);
//!     assert_eq!(b[1], 2);
//! }
//!
//! ```

#[cfg(test)]
mod tests;

pub mod heap;
use cty::c_long;
use heap::*;
// the hand writed native binding
pub use mimalloc_rust_sys as raw;
use raw::types::mi_heap_t;

use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    fmt::Debug,
    ops::Deref,
};

use crate::raw::{aligned_allocation::*, basic_allocation::*, heap::*, runtime_options::*};
/// The global allocator
pub struct GlobalMiMalloc;

impl Debug for GlobalMiMalloc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr = GlobalMiMalloc::get().heap.heap;
        let r = unsafe { &mut *ptr };
        f.write_fmt(format_args!("Global MiMalloc {:?}: {:?}", ptr, r))
    }
}
impl GlobalMiMalloc {
    /// replace the global allocator by a heap
    #[inline]
    pub fn replace_by<T: Deref<Target = *mut mi_heap_t>>(
        heap: &MiMallocHeap<T>,
    ) -> MiMallocHeapGlobal {
        MiMallocHeap {
            heap: GlobalHeap {
                heap: unsafe { mi_heap_set_default(*heap.heap.deref()) },
            },
        }
    }

    /// get the default heap type of the global allocator holds
    #[inline]
    pub fn get() -> MiMallocHeapGlobal {
        MiMallocHeap {
            heap: GlobalHeap {
                heap: unsafe { mi_heap_get_default() },
            },
        }
    }

    #[inline]
    pub fn option_disable(option: mi_option_t) {
        unsafe { mi_option_disable(option) }
    }

    #[inline]
    pub fn option_enable(option: mi_option_t) {
        unsafe { mi_option_enable(option) }
    }

    #[inline]
    pub fn option_get(option: mi_option_t) -> c_long {
        unsafe { mi_option_get(option) }
    }

    #[inline]
    pub fn option_is_enabled(option: mi_option_t) -> bool {
        unsafe { mi_option_is_enabled(option) }
    }

    #[inline]
    pub fn option_set(option: mi_option_t, value: c_long) {
        unsafe { mi_option_set(option, value) }
    }

    #[inline]
    pub fn option_set_default(option: mi_option_t, value: c_long) {
        unsafe { mi_option_set_default(option, value) }
    }

    #[inline]
    pub fn option_set_enabled(option: mi_option_t) {
        unsafe { mi_option_set_enabled(option) }
    }

    #[inline]
    pub fn option_set_enabled_default(option: mi_option_t) {
        unsafe { mi_option_set_enabled_default(option) }
    }
}
unsafe impl GlobalAlloc for GlobalMiMalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        mi_malloc_aligned(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        mi_zalloc_aligned(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        mi_realloc_aligned(ptr as *mut c_void, new_size, layout.align()) as *mut u8
    }
}
