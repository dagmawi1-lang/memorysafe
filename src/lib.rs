//! # MemorySafe
//! Each module demonstrates one memory-safety idea. The crate contains **no `unsafe` code**,
//! and the compiler enforces that with `#![forbid(unsafe_code)]`.
//!
//! The `compile_fail` examples in the docs are real tests: `cargo test` checks that the compiler
//! *rejects* them. They show the bugs Rust prevents at compile time.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod arena_graph;
pub mod errors;
pub mod ownership;
pub mod raii;
pub mod ring_buffer;
pub mod shared;
pub mod stack;
pub mod threads;
