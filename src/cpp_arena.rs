/// Pull C++ code into Rust
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("thesis/include/arena.h");

        fn benchmark_linked_list_arena(n: usize);
        fn benchmark_linked_list_manual(n: usize);
        fn benchmark_tree_arena(n: usize, arr: Vec<u32>);
        fn benchmark_tree_manual(arr: Vec<u32>);
    }
}