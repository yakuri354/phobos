# Allocation

The allocator used in phobos is liballoc, open source allocator written in C by Durand Miller

Crate `liballoc` provides Rust bindings to the allocator. It is located in `kernel/mm/alloc/liballoc`.

Rust global allocator API is implemented in file `kernel/mm/alloc/mod.rs`.