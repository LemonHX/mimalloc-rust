use crate::utils::BitField;

pub type mi_thread_free_t = usize;

pub type mi_page_kind_t = cty::c_uint;
pub const mi_page_kind_MI_PAGE_SMALL: mi_page_kind_t = 0;
pub const mi_page_kind_MI_PAGE_MEDIUM: mi_page_kind_t = 1;
pub const mi_page_kind_MI_PAGE_LARGE: mi_page_kind_t = 2;
pub const mi_page_kind_MI_PAGE_HUGE: mi_page_kind_t = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mi_page_t {
    pub segment_idx: u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: BitField<[u8; 1usize]>,
    pub capacity: u16,
    pub reserved: u16,
    pub flags: mi_page_flags_t,
    pub _bitfield_align_2: [u8; 0],
    pub _bitfield_2: BitField<[u8; 1usize]>,
    pub free: *mut mi_block_t,
    pub keys: [usize; 2usize],
    pub used: u32,
    pub xblock_size: u32,
    pub local_free: *mut mi_block_t,
    pub xthread_free: mi_thread_free_t,
    pub xheap: usize,
    pub next: *mut mi_page_t,
    pub prev: *mut mi_page_t,
}

impl mi_page_t {
    #[inline]
    pub fn segment_in_use(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_segment_in_use(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn is_reset(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_is_reset(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn is_committed(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_is_committed(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn is_zero_init(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_is_zero_init(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        segment_in_use: u8,
        is_reset: u8,
        is_committed: u8,
        is_zero_init: u8,
    ) -> BitField<[u8; 1usize]> {
        let mut _bitfield_unit: BitField<[u8; 1usize]> = Default::default();
        _bitfield_unit.set(0usize, 1u8, {
            let segment_in_use: u8 = unsafe { ::core::mem::transmute(segment_in_use) };
            segment_in_use as u64
        });
        _bitfield_unit.set(1usize, 1u8, {
            let is_reset: u8 = unsafe { ::core::mem::transmute(is_reset) };
            is_reset as u64
        });
        _bitfield_unit.set(2usize, 1u8, {
            let is_committed: u8 = unsafe { ::core::mem::transmute(is_committed) };
            is_committed as u64
        });
        _bitfield_unit.set(3usize, 1u8, {
            let is_zero_init: u8 = unsafe { ::core::mem::transmute(is_zero_init) };
            is_zero_init as u64
        });
        _bitfield_unit
    }
    #[inline]
    pub fn is_zero(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_2.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_is_zero(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_2.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn retire_expire(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_2.get(1usize, 7u8) as u8) }
    }
    #[inline]
    pub fn set_retire_expire(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_2.set(1usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(is_zero: u8, retire_expire: u8) -> BitField<[u8; 1usize]> {
        let mut _bitfield_unit: BitField<[u8; 1usize]> = Default::default();
        _bitfield_unit.set(0usize, 1u8, {
            let is_zero: u8 = unsafe { ::core::mem::transmute(is_zero) };
            is_zero as u64
        });
        _bitfield_unit.set(1usize, 7u8, {
            let retire_expire: u8 = unsafe { ::core::mem::transmute(retire_expire) };
            retire_expire as u64
        });
        _bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_block_t {
    pub next: usize,
}
pub type mi_delayed_t = cty::c_uint;
pub const mi_delayed_t_MI_USE_DELAYED_FREE: mi_delayed_t = 0;
pub const mi_delayed_t_MI_DELAYED_FREEING: mi_delayed_t = 1;
pub const mi_delayed_t_MI_NO_DELAYED_FREE: mi_delayed_t = 2;
pub const mi_delayed_t_MI_NEVER_DELAYED_FREE: mi_delayed_t = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub union mi_page_flags_t {
    pub full_aligned: u8,
    pub x: mi_page_flags_t_ty_1,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct mi_page_flags_t_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: BitField<[u8; 1usize]>,
}
impl mi_page_flags_t_ty_1 {
    #[inline]
    pub fn in_full(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_in_full(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn has_aligned(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_has_aligned(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(in_full: u8, has_aligned: u8) -> BitField<[u8; 1usize]> {
        let mut _bitfield_unit: BitField<[u8; 1usize]> = Default::default();
        _bitfield_unit.set(0usize, 1u8, {
            let in_full: u8 = unsafe { ::core::mem::transmute(in_full) };
            in_full as u64
        });
        _bitfield_unit.set(1usize, 1u8, {
            let has_aligned: u8 = unsafe { ::core::mem::transmute(has_aligned) };
            has_aligned as u64
        });
        _bitfield_unit
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mi_segment_t {
    pub memid: usize,
    pub mem_is_pinned: bool,
    pub mem_is_committed: bool,
    // atomic ptr
    pub abandoned_next: *mut mi_segment_t,
    pub next: *mut mi_segment_t,
    pub prev: *mut mi_segment_t,
    pub abandoned: usize,
    pub abandoned_visits: usize,
    pub used: usize,
    pub capacity: usize,
    pub segment_size: usize,
    pub segment_info_size: usize,
    pub cookie: usize,
    pub page_shift: usize,
    pub thread_id: usize,
    pub page_kind: mi_page_kind_t,
    pub pages: [mi_page_t; 1usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_page_queue_t {
    pub first: *mut mi_page_t,
    pub last: *mut mi_page_t,
    pub block_size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_random_ctx_t {
    pub input: [u32; 16usize],
    pub output: [u32; 16usize],
    pub output_available: cty::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_heap_t {
    pub tld: *mut mi_tld_t,
    pub pages_free_direct: [*mut mi_page_t; 130usize],
    pub pages: [mi_page_queue_t; 75usize],
    pub thread_delayed_free: mi_block_t,
    pub thread_id: usize,
    pub cookie: usize,
    pub keys: [usize; 2usize],
    pub random: mi_random_ctx_t,
    pub page_count: usize,
    pub page_retired_min: usize,
    pub page_retired_max: usize,
    pub next: *mut mi_heap_t,
    pub no_reclaim: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_stat_count_t {
    pub allocated: i64,
    pub freed: i64,
    pub peak: i64,
    pub current: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_stat_counter_t {
    pub total: i64,
    pub count: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_stats_t {
    pub segments: mi_stat_count_t,
    pub pages: mi_stat_count_t,
    pub reserved: mi_stat_count_t,
    pub committed: mi_stat_count_t,
    pub reset: mi_stat_count_t,
    pub page_committed: mi_stat_count_t,
    pub segments_abandoned: mi_stat_count_t,
    pub pages_abandoned: mi_stat_count_t,
    pub threads: mi_stat_count_t,
    pub normal: mi_stat_count_t,
    pub huge: mi_stat_count_t,
    pub giant: mi_stat_count_t,
    pub malloc: mi_stat_count_t,
    pub segments_cache: mi_stat_count_t,
    pub pages_extended: mi_stat_counter_t,
    pub mmap_calls: mi_stat_counter_t,
    pub commit_calls: mi_stat_counter_t,
    pub page_no_retire: mi_stat_counter_t,
    pub searches: mi_stat_counter_t,
    pub normal_count: mi_stat_counter_t,
    pub huge_count: mi_stat_counter_t,
    pub giant_count: mi_stat_counter_t,
    pub normal_bins: [mi_stat_count_t; 74usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_segment_queue_t {
    pub first: *mut mi_segment_t,
    pub last: *mut mi_segment_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_os_tld_t {
    pub region_idx: usize,
    pub stats: *mut mi_stats_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_segments_tld_s {
    pub small_free: mi_segment_queue_t,
    pub medium_free: mi_segment_queue_t,
    pub pages_reset: mi_page_queue_t,
    pub count: usize,
    pub peak_count: usize,
    pub current_size: usize,
    pub peak_size: usize,
    pub cache_count: usize,
    pub cache_size: usize,
    pub cache: *mut mi_segment_t,
    pub stats: *mut mi_stats_t,
    pub os: *mut mi_os_tld_t,
}
pub type mi_segments_tld_t = mi_segments_tld_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_tld_t {
    pub heartbeat: cty::c_ulonglong,
    pub recurse: bool,
    pub heap_backing: *mut mi_heap_t,
    pub heaps: *mut mi_heap_t,
    pub segments: mi_segments_tld_t,
    pub os: mi_os_tld_t,
    pub stats: mi_stats_t,
}
