use cty::{c_int, c_long};

pub type mi_option_t = c_int;

// stable options
pub const mi_option_show_errors: mi_option_t = 0; // print error messages
pub const mi_option_show_stats: mi_option_t = 1; // print statistics on termination
pub const mi_option_verbose: mi_option_t = 2; // print verbose messages

// the following options are experimental (see src/options.h)
pub const mi_option_eager_commit: mi_option_t = 3; // eager commit segments? (after `eager_commit_delay` segments) (=1)
pub const mi_option_arena_eager_commit: mi_option_t = 4; // eager commit arenas? Use 2 to enable just on overcommit systems (=2)
pub const mi_option_purge_decommits: mi_option_t = 5; // should a memory purge decommit (or only reset) (=1)
pub const mi_option_allow_large_os_pages: mi_option_t = 6; // allow large (2MiB) OS pages; implies eager commit
pub const mi_option_reserve_huge_os_pages: mi_option_t = 7; // reserve N huge OS pages (1GiB/page) at startup
pub const mi_option_reserve_huge_os_pages_at: mi_option_t = 8; // reserve huge OS pages at a specific NUMA node
pub const mi_option_reserve_os_memory: mi_option_t = 9; // reserve specified amount of OS memory in an arena at startup
pub const mi_option_deprecated_segment_cache: mi_option_t = 10;
pub const mi_option_deprecated_page_reset: mi_option_t = 11;
pub const mi_option_abandoned_page_purge: mi_option_t = 12; // immediately purge delayed purges on thread termination
pub const mi_option_deprecated_segment_reset: mi_option_t = 13;
pub const mi_option_eager_commit_delay: mi_option_t = 14;
pub const mi_option_purge_delay: mi_option_t = 15; // memory purging is delayed by N milli seconds; use 0 for immediate purging or -1 for no purging at all.
pub const mi_option_use_numa_nodes: mi_option_t = 16; // 0 = use all available numa nodes; otherwise use at most N nodes.
pub const mi_option_limit_os_alloc: mi_option_t = 17; // 1 = do not use OS memory for allocation (but only programmatically reserved arenas)
pub const mi_option_os_tag: mi_option_t = 18; // tag used for OS logging (macOS only for now)
pub const mi_option_max_errors: mi_option_t = 19; // issue at most N error messages
pub const mi_option_max_warnings: mi_option_t = 20; // issue at most N warning messages
pub const mi_option_max_segment_reclaim: mi_option_t = 21;
pub const mi_option_destroy_on_exit: mi_option_t = 22; // if set; release all memory on exit; sometimes used for dynamic unloading but can be unsafe.
pub const mi_option_arena_reserve: mi_option_t = 23; // initial memory size in KiB for arena reservation (1GiB on 64-bit)
pub const mi_option_arena_purge_mult: mi_option_t = 24;
pub const mi_option_purge_extend_delay: mi_option_t = 25;
pub const _mi_option_last: mi_option_t = 26;

// legacy option names
pub const mi_option_large_os_pages: mi_option_t = mi_option_allow_large_os_pages;
pub const mi_option_eager_region_commit: mi_option_t = mi_option_arena_eager_commit;
pub const mi_option_reset_decommits: mi_option_t = mi_option_purge_decommits;
pub const mi_option_reset_delay: mi_option_t = mi_option_purge_delay;
pub const mi_option_abandoned_page_reset: mi_option_t = mi_option_abandoned_page_purge;

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
