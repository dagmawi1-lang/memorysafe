# MemorySafe roadmap

## Done (v1.0.0)
- [x] Demonstrate ownership and borrowing (with compile-fail tests)
- [x] Safe memory management (Box, Rc/Weak, RAII, arena)
- [x] Error handling (custom error types, `?`, checked arithmetic)
- [x] Safe data structures (stack, ring buffer, graph)
- [x] Documented memory-safety concepts (README, CONCEPTS.md, rustdoc)
- [x] 24 tests, clippy clean, `#![forbid(unsafe_code)]`

## Next
- [ ] Side-by-side C program with a real use-after-free, caught with AddressSanitizer, next to the Rust version that won't compile
- [ ] A doubly linked list using `Rc<RefCell<>>` + `Weak`, compared with the arena version
- [ ] Criterion benchmarks: arena vs `Box` allocations
- [ ] Miri run in CI
