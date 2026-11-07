//! `cargo run` walks through every concept with a short explanation and live output.
use memorysafe::{
    arena_graph::Graph, errors, ownership, raii, ring_buffer::RingBuffer, shared::TreeNode,
    stack::Stack, threads,
};

fn section(title: &str, why: &str) {
    println!("\n━━ {title} ━━\n{why}");
}

fn main() {
    println!("MemorySafe: memory safety in Rust, demonstrated (0 lines of unsafe code)");

    section("1. Ownership & borrowing", "Each value has one owner; borrows are checked at compile time (no use-after-free, no iterator invalidation).");
    let mut s = String::from("borrow checker");
    println!("vowels in {s:?}: {}", ownership::count_vowels(&s));
    ownership::shout(&mut s);
    println!("after a mutable borrow: {s}");
    println!(
        "longest(\"stack\", \"heap\") = {}",
        ownership::longest("stack", "heap")
    );
    println!(
        "window(&[1,2,3], 2, 5) = {:?}  (C would read past the buffer)",
        ownership::window(&[1, 2, 3], 2, 5)
    );

    section(
        "2. Safe linked stack",
        "Option<Box<Node>> replaces nullable raw pointers; popping frees the node automatically.",
    );
    let mut st = Stack::new();
    for w in ["alloc", "use", "free"] {
        st.push(w);
    }
    let before: Vec<_> = st.iter().copied().collect(); // finish the shared borrow…
    let popped = st.pop(); // …before taking a mutable one (the compiler insisted; see README)
    println!("stack top→bottom: {before:?}, pop → {popped:?}");

    section(
        "3. Ring buffer",
        "Indices are wrapped and checked; a full buffer is an error value, not an overflow.",
    );
    let mut rb = RingBuffer::with_capacity(3);
    for i in 1..=4 {
        match rb.push(i) {
            Ok(()) => println!("pushed {i}"),
            Err(e) => println!("buffer full, got {:?} back", e.0),
        }
    }

    section(
        "4. Graphs without pointers",
        "Nodes in an arena + typed indices: cycles can't dangle or leak.",
    );
    let mut g = Graph::new();
    let [a, b, c] = ["home", "school", "library"].map(|n| g.add(n));
    g.connect(a, b);
    g.connect(b, c);
    g.connect(c, a);
    let path: Vec<_> = g
        .shortest_path(a, c)
        .unwrap()
        .into_iter()
        .map(|n| *g.get(n))
        .collect();
    println!("shortest path home → library: {}", path.join(" → "));

    section(
        "5. Rc / Weak",
        "Shared ownership with weak parent links, so trees don't form leaking reference cycles.",
    );
    let root = TreeNode::new("root");
    let leaf = TreeNode::new("leaf");
    TreeNode::adopt(&root, leaf.clone());
    println!(
        "path = {}, root strong refs = {}",
        TreeNode::path(&leaf),
        std::rc::Rc::strong_count(&root)
    );

    section("6. Errors as values", "Result + custom error types: every failure is handled explicitly, with no null or unchecked exceptions.");
    for cfg in ["port = 8080", "port = 99999", "port: 80", "workers = 2"] {
        match errors::parse_config(cfg) {
            Ok(c) => println!("{cfg:14} → ok {c:?}"),
            Err(e) => println!("{cfg:14} → error: {e}"),
        }
    }
    println!(
        "buffer_size(usize::MAX/2, 4) = {:?}  (overflow caught)",
        errors::buffer_size(usize::MAX / 2, 4)
    );

    section(
        "7. RAII",
        "Resources are released exactly once, in reverse order, even on early return.",
    );
    let log: raii::Log = Default::default();
    let _ = raii::work(true, &log);
    println!("{}", log.borrow().join(" → "));

    section(
        "8. Fearless concurrency",
        "Arc<Mutex<T>>, channels, and scoped threads; data races don't compile.",
    );
    println!(
        "8 threads × 10,000 increments = {}",
        threads::parallel_count(8, 10_000)
    );
    println!(
        "sum 1..=1000 on 2 scoped threads = {}",
        threads::parallel_sum(&(1..=1000).collect::<Vec<u64>>())
    );
}
