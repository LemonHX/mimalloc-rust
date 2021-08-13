use cty::{c_int, c_long};

pub type mi_option_t = c_int;

pub const mi_option_show_errors: mi_option_t = 0;
pub const mi_option_show_stats: mi_option_t = 1;
pub const mi_option_verbose: mi_option_t = 2;
// the following options are experimental
pub const mi_option_eager_commit: mi_option_t = 3;
pub const mi_option_eager_region_commit: mi_option_t = 4;
pub const mi_option_reset_decommits: mi_option_t = 5;
// implies eager commit
pub const mi_option_large_os_pages: mi_option_t = 6;
pub const mi_option_reserve_huge_os_pages: mi_option_t = 7;
pub const mi_option_reserve_os_memory: mi_option_t = 8;
pub const mi_option_segment_cache: mi_option_t = 9;
pub const mi_option_page_reset: mi_option_t = 10;
pub const mi_option_abandoned_page_reset: mi_option_t = 11;
pub const mi_option_segment_reset: mi_option_t = 12;
pub const mi_option_eager_commit_delay: mi_option_t = 13;
pub const mi_option_reset_delay: mi_option_t = 14;
pub const mi_option_use_numa_nodes: mi_option_t = 15;
pub const mi_option_limit_os_alloc: mi_option_t = 16;
pub const mi_option_os_tag: mi_option_t = 17;
pub const mi_option_max_errors: mi_option_t = 18;
pub const mi_option_max_warnings: mi_option_t = 19;

extern "C" {
    pub fn mi_option_disable(option: mi_option_t);
    pub fn mi_option_enable(option: mi_option_t);
    pub fn mi_option_get(option: mi_option_t) -> c_long;
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;
    pub fn mi_option_set(option: mi_option_t, value: c_long);
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
    pub fn mi_option_set_enabled(option: mi_option_t);
    pub fn mi_option_set_enabled_default(option: mi_option_t);
}
