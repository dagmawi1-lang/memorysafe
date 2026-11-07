# Test results — 2026-11-07
```
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```
`cargo clippy -- -D warnings`: clean · `cargo fmt --check`: clean · rustc 1.97.0 (2d8144b78 2026-07-07)

## `cargo run` output
```
MemorySafe: memory safety in Rust, demonstrated (0 lines of unsafe code)

━━ 1. Ownership & borrowing ━━
Each value has one owner; borrows are checked at compile time (no use-after-free, no iterator invalidation).
vowels in "borrow checker": 4
after a mutable borrow: BORROW CHECKER!
longest("stack", "heap") = stack
window(&[1,2,3], 2, 5) = None  (C would read past the buffer)

━━ 2. Safe linked stack ━━
Option<Box<Node>> replaces nullable raw pointers; popping frees the node automatically.
stack top→bottom: ["free", "use", "alloc"], pop → Some("free")

━━ 3. Ring buffer ━━
Indices are wrapped and checked; a full buffer is an error value, not an overflow.
pushed 1
pushed 2
pushed 3
buffer full, got 4 back

━━ 4. Graphs without pointers ━━
Nodes in an arena + typed indices: cycles can't dangle or leak.
shortest path home → library: home → school → library

━━ 5. Rc / Weak ━━
Shared ownership with weak parent links, so trees don't form leaking reference cycles.
path = root/leaf, root strong refs = 1

━━ 6. Errors as values ━━
Result + custom error types: every failure is handled explicitly, with no null or unchecked exceptions.
port = 8080    → ok ServerConfig { port: 8080, workers: 4 }
port = 99999   → error: port = 99999 is outside 1..=65535
port: 80       → error: line 1: expected `key = value`
workers = 2    → error: missing required key `port`
buffer_size(usize::MAX/2, 4) = None  (overflow caught)

━━ 7. RAII ━━
Resources are released exactly once, in reverse order, even on early return.
acquire db → acquire file → release file → release db

━━ 8. Fearless concurrency ━━
Arc<Mutex<T>>, channels, and scoped threads; data races don't compile.
8 threads × 10,000 increments = 80000
sum 1..=1000 on 2 scoped threads = 500500
```
