//! # mi-malloc
//! [documentation](https**://microsoft.github.io/mimalloc/index.html)
//! - **small and consistent**: the library is about 8k LOC using simple and consistent data structures. This makes it very suitable to integrate and adapt in other projects. For runtime systems it provides hooks for a monotonic heartbeat and deferred freeing (for bounded worst-case times with reference counting).
//! - **free list sharding**: instead of one big free list (per size class) we have many smaller lists per "mimalloc page" which reduces fragmentation and increases locality – things that are allocated close in time get allocated close in memory. (A mimalloc page contains blocks of one size class and is usually 64KiB on a 64-bit system).
//! - **free list multi-sharding**: the big idea! Not only do we shard the free list per mimalloc page, but for each page we have multiple free lists. In particular, there is one list for thread-local free operations, and another one for concurrent free operations. Free-ing from another thread can now be a single CAS without needing sophisticated coordination between threads. Since there will be thousands of separate free lists, contention is naturally distributed over the heap, and the chance of contending on a single location will be low – this is quite similar to randomized algorithms like skip lists where adding a random oracle removes the need for a more complex algorithm.
//! - **eager page reset**: when a "page" becomes empty (with increased chance due to free list sharding) the memory is marked to the OS as unused ("reset" or "purged") reducing (real) memory pressure and fragmentation, especially in long running programs.
//! - **secure**: mimalloc can be build in secure mode, adding guard pages, randomized allocation, encrypted free lists, etc. to protect against various heap vulnerabilities. The performance penalty is only around 3% on average over our benchmarks.
//! - **first-class heaps**: efficiently create and use multiple heaps to allocate across different regions. A heap can be destroyed at once instead of deallocating each object separately.
//! - **bounded**: it does not suffer from blowup, has bounded worst-case allocation times (wcat), bounded space overhead (~0.2% meta-data, with at most 12.5% waste in allocation sizes), and has no internal points of contention using only atomic operations.
//! - **fast**: In our benchmarks (see below), mimalloc outperforms all other leading allocators (jemalloc, tcmalloc, Hoard, etc), and usually uses less memory (up to 25% more in the worst case). A nice property is that it does consistently well over a wide range of benchmarks.

#![no_std]
#![allow(nonstandard_style)]

pub mod aligned_allocation;
pub mod basic_allocation;
pub mod extended_functions;
pub mod heap;
pub mod runtime_options;
pub mod types;
pub mod utils;
