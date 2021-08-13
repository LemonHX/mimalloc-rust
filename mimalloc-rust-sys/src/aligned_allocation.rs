use cty::c_void;

//Doc: https://microsoft.github.io/mimalloc/group__aligned.html

extern "C" {
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_malloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_zalloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;
    pub fn mi_calloc_aligned(count: usize, size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_calloc_aligned_at(
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
    pub fn mi_realloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;
    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
