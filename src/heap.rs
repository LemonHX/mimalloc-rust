//! First-class heaps that can be destroyed in one go.
//!
//! [furthur documentation](https://microsoft.github.io/mimalloc/group__heap.html#details)
#[cfg(feature = "unstable")]
use crate::raw::basic_allocation::mi_free;
use crate::raw::{heap::*, types::mi_heap_t};
#[cfg(feature = "unstable")]
use core::{
    alloc::*,
    ptr::{slice_from_raw_parts_mut, NonNull},
};
use core::{ffi::c_void, fmt::Debug, ops::Deref};

/// Heap type used for allocator API
pub struct MiMallocHeap<T: Deref<Target = *mut mi_heap_t>> {
    pub heap: T,
}

impl<T: Deref<Target = *mut mi_heap_t>> MiMallocHeap<T> {
    #[inline]
    pub fn new(heap: T) -> Self {
        Self { heap }
    }
}

impl<T> Debug for MiMallocHeap<T>
where
    T: Deref<Target = *mut mi_heap_t> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.heap))
    }
}

impl<T: Deref<Target = *mut mi_heap_t>> From<T> for MiMallocHeap<T> {
    fn from(heap: T) -> Self {
        Self { heap }
    }
}

#[cfg(feature = "unstable")]
unsafe impl<T: Deref<Target = *mut mi_heap_t>> Allocator for MiMallocHeap<T> {
    #[inline]
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        unsafe {
            let mem = mi_heap_malloc_aligned(*self.heap.deref(), layout.size(), layout.align());
            match NonNull::new(mem) {
                Some(mem) => Ok(NonNull::new_unchecked(slice_from_raw_parts_mut(
                    mem.as_ptr() as *mut _,
                    layout.size(),
                ))),
                None => Err(AllocError),
            }
        }
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, _layout: Layout) {
        mi_free(ptr.as_ptr() as *mut _)
    }

    #[inline]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        unsafe {
            let mem = mi_heap_zalloc_aligned(*self.heap.deref(), layout.size(), layout.align());
            match NonNull::new(mem) {
                Some(mem) => Ok(NonNull::new_unchecked(slice_from_raw_parts_mut(
                    mem.as_ptr() as *mut _,
                    layout.size(),
                ))),
                None => Err(AllocError),
            }
        }
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );

        let mem = mi_heap_realloc_aligned(
            *self.heap.deref(),
            ptr.as_ptr() as *mut _,
            new_layout.size(),
            new_layout.align(),
        );
        match NonNull::new(mem) {
            Some(mem) => Ok(NonNull::new_unchecked(slice_from_raw_parts_mut(
                mem.as_ptr() as *mut _,
                new_layout.size(),
            ))),
            None => Err(AllocError),
        }
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );

        let mem = mi_heap_rezalloc_aligned(
            *self.heap.deref(),
            ptr.as_ptr() as *mut _,
            new_layout.size(),
            new_layout.align(),
        );
        match NonNull::new(mem) {
            Some(mem) => Ok(NonNull::new_unchecked(slice_from_raw_parts_mut(
                mem.as_ptr() as *mut _,
                new_layout.size(),
            ))),
            None => Err(AllocError),
        }
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );

        let mem = mi_heap_realloc_aligned(
            *self.heap.deref(),
            ptr.as_ptr() as *mut _,
            new_layout.size(),
            new_layout.align(),
        );
        match NonNull::new(mem) {
            Some(mem) => Ok(NonNull::new_unchecked(slice_from_raw_parts_mut(
                mem.as_ptr() as *mut _,
                new_layout.size(),
            ))),
            None => Err(AllocError),
        }
    }

    #[inline]
    fn by_ref(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

/// A custom function which visits the Heap
pub trait HeapVisitor<VisitorName, T: Deref<Target = *mut mi_heap_t>>
where
    Self: Sized,
{
    fn visitor(
        &mut self,
        heap: &mi_heap_t,
        area: &mi_heap_area_t,
        block: *mut c_void,
        size: usize,
    ) -> bool;

    fn visit(&mut self, heap: &MiMallocHeap<T>) {
        unsafe {
            let heap: *mut mi_heap_t = *heap.heap.deref();
            mi_heap_visit_blocks(
                heap as *const mi_heap_t,
                false,
                Some(visit_handler::<VisitorName, T, Self>),
                self as *mut Self as *mut c_void,
            );
        }
    }
}

/// the default Global Heap Type
#[derive(Debug, PartialEq, Eq)]
pub struct GlobalHeap {
    pub heap: *mut mi_heap_t,
}

impl Deref for GlobalHeap {
    type Target = *mut mi_heap_t;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}
/// the default Global Heap Type Alias
pub type MiMallocHeapGlobal = MiMallocHeap<GlobalHeap>;

#[inline]
unsafe extern "C" fn visit_handler<
    VisitorName,
    T: Deref<Target = *mut mi_heap_t>,
    Visitor: HeapVisitor<VisitorName, T>,
>(
    heap: *const mi_heap_t,
    area: *const mi_heap_area_t,
    block: *mut c_void,
    size: usize,
    args: *mut c_void,
) -> bool {
    let visitor = &mut *(args as *mut Visitor);
    Visitor::visitor(visitor, &*heap, &*area, block, size)
}

/// a macro which could only be used in single thread condition
#[macro_export]
macro_rules! with_heap {
    ($heap: ty, $do: expr) => {{
        let heap = MiMallocHeap::from(<$heap>::new());
        let global = GlobalMiMalloc::replace_by(&heap);
        debug_assert!(GlobalMiMalloc::get().heap != global.heap);
        let res = { $do };
        GlobalMiMalloc::replace_by(&global);
        debug_assert!(GlobalMiMalloc::get().heap == global.heap);
        (res, heap)
    }};
}
