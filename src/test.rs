use std::{ffi::c_void, mem::ManuallyDrop, println};

use mimalloc_sys::{
    heap::{mi_heap_area_t, mi_heap_delete},
    runtime_options::mi_option_show_stats,
    types::mi_heap_t,
};

use crate::{
    heap::{HeapVisitor, MiMallocHeap},
    sys::heap::mi_heap_new,
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

struct TestHeap {
    heap: *mut mi_heap_t,
}
use std::ops::Deref;
impl Deref for TestHeap {
    type Target = *mut mi_heap_t;

    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}

impl TestHeap {
    fn new() -> Self {
        Self {
            heap: unsafe { mi_heap_new() },
        }
    }
}

impl Drop for TestHeap {
    fn drop(&mut self) {
        unsafe { mi_heap_delete(self.heap) }
    }
}

#[derive(Default, Debug)]
struct LeakDetector {
    in_used: usize,
    in_used_size: usize,
}

impl Drop for LeakDetector {
    fn drop(&mut self) {
        if self.in_used != 0 && self.in_used_size != 0 {
            panic!("Memory Leaks with information {:?}", self);
        }
    }
}
enum General {}
impl HeapVisitor<General, TestHeap> for LeakDetector {
    fn visitor(
        &mut self,
        _heap: &mi_heap_t,
        area: &mi_heap_area_t,
        _block: *mut c_void,
        _size: usize,
    ) -> bool {
        // println!("{:?}", area);
        // println!("{:?}", size);
        self.in_used += area.used;
        self.in_used_size += area.committed - area.used;
        true
    }
}

#[test]
#[should_panic]
fn test_leak_detector() {
    let (_res, heap) = with_heap!(TestHeap, unsafe {
        let mut b: ManuallyDrop<Vec<u8>> = ManuallyDrop::new(vec![0; 114514]);
        let mut _leak1: ManuallyDrop<Vec<u8>> = ManuallyDrop::new(vec![0; 114514]);
        let mut _leak2: ManuallyDrop<Vec<u8>> = ManuallyDrop::new(vec![0; 1]);
        ManuallyDrop::drop(&mut b);
    });
    LeakDetector::default().visit(&heap);
}
