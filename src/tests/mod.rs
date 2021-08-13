mod heap;

use crate::raw::runtime_options::mi_option_show_stats;

use crate::GlobalMiMalloc;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

#[test]
fn test_malloc() {
    GlobalMiMalloc::option_enable(mi_option_show_stats);
    let _vec: Vec<u8> = vec![0; 114514];
    println!("mimalloc: \n{:?}", GLOBAL_MIMALLOC);
}
