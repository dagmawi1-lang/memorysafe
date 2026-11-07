//! Fearless concurrency: the type system prevents data races at compile time.
//!
//! `Rc` is not thread-safe (`!Send`), so sending it to another thread is rejected:
//! ```compile_fail
//! use std::rc::Rc;
//! let rc = Rc::new(5);
//! std::thread::spawn(move || println!("{rc}")); // error[E0277]: `Rc<i32>` cannot be sent between threads safely
//! ```
//! Mutating shared data without a lock is also rejected:
//! ```compile_fail
//! let mut counter = 0;
//! let handles: Vec<_> = (0..2).map(|_| std::thread::spawn(|| counter += 1)).collect(); // error: closure may outlive `counter`
//! ```

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Many threads increment one counter through `Arc<Mutex<_>>`. The result is always exact.
pub fn parallel_count(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments {
                    *c.lock().expect("lock poisoned") += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("thread panicked");
    }
    let total = *counter.lock().unwrap();
    total
}

/// Workers send results over a channel. Ownership of each message moves to the receiver.
pub fn parallel_squares(inputs: Vec<u64>) -> Vec<u64> {
    let (tx, rx) = mpsc::channel();
    let chunks: Vec<Vec<u64>> = inputs.chunks(4).map(|c| c.to_vec()).collect();
    for chunk in chunks {
        let tx = tx.clone();
        thread::spawn(move || {
            for x in chunk {
                tx.send(x * x).expect("receiver alive");
            }
        });
    }
    drop(tx); // close our sender so the receiving loop ends when all workers finish
    let mut out: Vec<u64> = rx.into_iter().collect();
    out.sort_unstable();
    out
}

/// Scoped threads can borrow local data because the compiler proves they finish before it is dropped.
pub fn parallel_sum(data: &[u64]) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().sum::<u64>());
        let b = s.spawn(|| right.iter().sum::<u64>());
        a.join().unwrap() + b.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutex_counter_is_exact() {
        assert_eq!(parallel_count(8, 10_000), 80_000);
    }

    #[test]
    fn channels_and_scoped_threads() {
        assert_eq!(
            parallel_squares((1..=10).collect()),
            vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
        );
        let data: Vec<u64> = (1..=1000).collect();
        assert_eq!(parallel_sum(&data), 500_500);
    }
}
