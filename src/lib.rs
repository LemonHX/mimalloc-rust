#![feature(allocator_api)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod test;

pub mod heap;
use cty::c_long;
use heap::GlobalHeap;
use heap::MiMallocHeap;
use heap::MiMallocHeapGlobal;
pub use mimalloc_sys as sys;
use sys::types::mi_heap_t;

use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    fmt::Debug,
    ops::Deref,
};

use mimalloc_sys::{aligned_allocation::*, basic_allocation::*, heap::*, runtime_options::*};

pub struct GlobalMiMalloc;

impl Debug for GlobalMiMalloc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr = GlobalMiMalloc::get().heap.heap;
        let r = unsafe { &mut *ptr };
        f.write_fmt(format_args!("Global MiMalloc {:?}: {:?}", ptr, r))
    }
}
impl GlobalMiMalloc {
    pub fn replace_by<T: Deref<Target = *mut mi_heap_t>>(
        heap: &MiMallocHeap<T>,
    ) -> MiMallocHeapGlobal {
        MiMallocHeap {
            heap: GlobalHeap {
                heap: unsafe { mi_heap_set_default(*heap.heap.deref()) },
            },
        }
    }
}
impl GlobalMiMalloc {
    pub fn get() -> MiMallocHeapGlobal {
        MiMallocHeap {
            heap: GlobalHeap {
                heap: unsafe { mi_heap_get_default() },
            },
        }
    }
    pub fn option_disable(option: mi_option_t) {
        unsafe { mi_option_disable(option) }
    }
    pub fn option_enable(option: mi_option_t) {
        unsafe { mi_option_enable(option) }
    }
    pub fn option_get(option: mi_option_t) -> c_long {
        unsafe { mi_option_get(option) }
    }
    pub fn option_is_enabled(option: mi_option_t) -> bool {
        unsafe { mi_option_is_enabled(option) }
    }
    pub fn option_set(option: mi_option_t, value: c_long) {
        unsafe { mi_option_set(option, value) }
    }
    pub fn option_set_default(option: mi_option_t, value: c_long) {
        unsafe { mi_option_set_default(option, value) }
    }
    pub fn option_set_enabled(option: mi_option_t) {
        unsafe { mi_option_set_enabled(option) }
    }
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
