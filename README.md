# rust-parallel-options

This demonstrates many options for executing a pair of functions in Rust including:

1. serially: one at a time
1. concurrently: taking turns using a single thread
1. parallel using operating system threads
1. parallel using tasks (a.k.a. green threads)

These options are demonstrated using the `std`, `async_std`, and `tokio` crates.
Note that options 2 and 4 are not possible using only the `std` crate.
