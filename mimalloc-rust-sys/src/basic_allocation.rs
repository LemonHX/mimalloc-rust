use cty::{c_char, c_void};
// Doc: https://microsoft.github.io/mimalloc/group__malloc.html
extern "C" {
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;
    pub fn mi_expand(p: *mut c_void, size: usize) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);
    pub fn mi_malloc(size: usize) -> *mut c_void;
    pub fn mi_mallocn(count: usize, size: usize) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;
    pub fn mi_reallocf(p: *mut c_void, newsize: usize) -> *mut c_void;
    pub fn mi_reallocn(p: *mut c_void, count: usize, size: usize) -> *mut c_void;
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;
    pub fn mi_recalloc(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;
    pub fn mi_strdup(s: *const c_char) -> *mut c_char;
    pub fn mi_strndup(s: *const c_char, n: usize) -> *mut c_char;
    pub fn mi_zalloc(size: usize) -> *mut c_void;
}
