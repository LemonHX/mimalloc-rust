mod heap;
use std::{ffi::c_void, mem::ManuallyDrop, println};

use crate::raw::{
    heap::{mi_heap_area_t, mi_heap_delete},
    runtime_options::mi_option_show_stats,
    types::mi_heap_t,
};

use crate::{
    heap::{HeapVisitor, MiMallocHeap},
    raw::heap::mi_heap_new,
    with_heap, GlobalMiMalloc,
};

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

#[test]
fn test_malloc() {
    GlobalMiMalloc::option_enable(mi_option_show_stats);
    let _vec: Vec<u8> = vec![0; 114514];
    println!("mimalloc: \n{:?}", GLOBAL_MIMALLOC);
}
