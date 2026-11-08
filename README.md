# 🦀 MemorySafe — a Rust systems-programming demonstrator

MemorySafe is a library and walkthrough showing how Rust removes whole classes of memory bugs (use-after-free, double free, dangling pointers, buffer overflows, data races, null dereferences) **at compile time and without a garbage collector**. The crate contains **zero `unsafe` code**, enforced by `#![forbid(unsafe_code)]`.

## My role & contributions

**Solo developer · AI-assisted**

- Picked which memory-safety bugs to demonstrate and wrote compile-fail tests proving Rust rejects them
- Built a stack, ring buffer, and arena graph with zero unsafe code
- Hit a real borrow-checker error (iterator invalidation) in my own demo and documented the fix

**Language:** Rust (2021 edition, standard library only)
**Shows:** ownership and borrowing, lifetimes, safe data structures, RAII, error handling, and concurrency.

## Run
```bash
cargo run          # guided tour of all 8 concepts with live output
cargo test         # 18 unit + 1 integration + 5 doc tests
cargo doc --open   # API docs with the explanations
cargo clippy -- -D warnings   # clean
```

## What each module demonstrates
| Module | Concept | Bug class it prevents |
|---|---|---|
| `ownership` | moves, `&` / `&mut` borrows, lifetimes, bounds-checked slices | use-after-free, dangling refs, iterator invalidation, buffer over-read |
| `stack` | linked list with `Option<Box<Node>>`, iterators tied to lifetimes, iterative `Drop` (1M nodes, no stack overflow) | null deref, double free, leaks |
| `ring_buffer` | fixed-capacity generic circular buffer; a full buffer returns `Err(Full(value))` | buffer overflow |
| `arena_graph` | cyclic graph using an arena + typed `NodeId` handles, BFS shortest path | dangling pointers in cyclic structures |
| `shared` | `Rc`, `RefCell`, `Weak` parent links; test proves parents are freed | reference-cycle leaks, aliasing bugs |
| `errors` | custom error enum, `Display`/`Error`, `?`, `checked_mul` overflow detection | unchecked errors, integer overflow in size math |
| `raii` | `Drop` guards released exactly once, in reverse order, even on early return or **panic** | leaks, double release |
| `threads` | `Arc<Mutex<T>>`, channels, scoped threads | data races |

## Compile-fail tests: bugs that don't compile
The doc tests marked `compile_fail` pass only if the compiler **rejects** the code:
- using a `String` after moving it (E0382)
- returning a reference to a local variable
- pushing to a `Vec` while holding a reference into it (E0502)
- sending an `Rc` to another thread (E0277: not `Send`)
- mutating a captured counter from multiple threads

While writing `main.rs` I made the iterator-invalidation mistake for real: I printed `st.iter()` and called `st.pop()` in the same `println!`. The compiler stopped it with E0502, the same error shown in the docs. In C++ that code would compile and read freed memory.

See `docs/CONCEPTS.md` for a C/C++ vs Rust comparison of each bug.

## License

**Permission required.** © 2026 Dagmawi Melaku. You need my written permission before using, copying, or modifying this code, and any approved use must credit me with a link to this repository. Viewing, forking on GitHub, and running it to judge or review a submission are allowed without asking. To ask, [open an issue](../../issues/new). See [LICENSE](LICENSE) for the full terms.
