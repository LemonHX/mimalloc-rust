use cty::{c_char, c_int, c_ulonglong, c_void};
// Doc: https://microsoft.github.io/mimalloc/group__malloc.html
pub const MI_SMALL_SIZE_MAX: usize = 128 * core::mem::size_of::<*mut c_void>();
pub type mi_deferred_free_fun =
    Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;
pub type mi_error_fun = Option<unsafe extern "C" fn(code: c_int, arg: *mut c_void)>;
extern "C" {
    pub fn mi_collect(force: bool);
    pub fn mi_good_size(size: usize) -> usize;
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;
    pub fn mi_malloc_small(size: usize) -> *mut c_void;
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );
    pub fn mi_register_deferred_free(out: mi_deferred_free_fun, arg: *mut c_void);
    pub fn mi_register_error(out: mi_error_fun, arg: *mut c_void);
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_reserve_huge_os_pages_at(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
    ) -> c_int;
    pub fn mi_reserve_huge_os_pages_interleave(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
    ) -> c_int;
    pub fn mi_stats_print(_: *mut c_void);
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_stats_reset();
    pub fn mi_stats_merge();
    pub fn mi_thread_init();
    pub fn mi_thread_done();
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_usable_size(p: *const c_void) -> usize;
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;
}
