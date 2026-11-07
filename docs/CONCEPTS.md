# Memory-safety bugs: C/C++ vs Rust

| Bug | C/C++ example | What Rust does |
|---|---|---|
| **Use after free** | `char* p = malloc(8); free(p); p[0] = 'x';` | Ownership: when the owner is dropped, every borrow must already be gone. Checked at compile time. |
| **Double free** | `free(p); free(p);` | Exactly one owner, and `Drop` runs once. You can't call `drop` on a value twice (it's moved into `drop`). |
| **Dangling pointer** | returning `&local` | Lifetimes: `fn f() -> &String { &s }` is rejected. |
| **Iterator invalidation** | `for (auto& x : v) v.push_back(x);` | Aliasing XOR mutation: you can't take `&mut v` while `&v` is alive. |
| **Buffer overflow** | `buf[len]` or `memcpy` past the end | Indexing is bounds-checked (it panics instead of corrupting memory); `get()` returns `Option`. |
| **Null dereference** | `node->next->value` | No null references. `Option<T>` forces you to handle the empty case. |
| **Integer overflow** | `size_t n = count * size;` wraps | Debug builds panic on overflow (this project enables it in release too); `checked_*` returns `None`. |
| **Data race** | two threads `counter++` | `Send`/`Sync` traits: shared mutable state needs `Mutex`/atomics, or it doesn't compile. |
| **Leaks from cycles** | `shared_ptr` cycles | Still possible with `Rc`, but `Weak` breaks cycles. Arenas with indices avoid them entirely. |

## What Rust does *not* guarantee
- Leaks are "safe" (`Rc` cycles, `std::mem::forget`), so they still need design care, as the `shared` module shows.
- Logic bugs, deadlocks, and panics (out-of-bounds indexing panics; it doesn't corrupt memory, but the program still stops).
- `unsafe` blocks can break the rules. This crate forbids them.
