use cty::{c_char, c_void};

use crate::types::mi_heap_t;

// Doc: https://microsoft.github.io/mimalloc/group__heap.html

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mi_heap_area_t {
    pub blocks: *mut c_void,
    pub reserved: usize,
    pub committed: usize,
    pub used: usize,
    pub block_size: usize,
}

pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: usize,
        arg: *mut c_void,
    ) -> bool,
>;

extern "C" {
    pub fn mi_heap_new() -> *mut mi_heap_t;
    pub fn mi_heap_delete(heap: *mut mi_heap_t);
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;
    pub fn mi_heap_get_default() -> *mut mi_heap_t;
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: usize) -> *mut c_char;
    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: usize,
        size: usize,
    ) -> *mut c_void;
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    // Zero initialized re-allocation
    // Doc: https://microsoft.github.io/mimalloc/group__zeroinit.html

    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
    ) -> *mut c_void;
    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    // Heap Introspection
    // Doc: https://microsoft.github.io/mimalloc/group__analysis.html

    pub fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *const c_void) -> bool;
    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void) -> bool;
    pub fn mi_check_owned(p: *const c_void) -> bool;
    pub fn mi_heap_visit_blocks(
        heap: *const mi_heap_t,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;
}
