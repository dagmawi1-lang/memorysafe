//! RAII: resources are released deterministically when their owner goes out of scope, even on early
//! return or panic. This is how Rust avoids leaks and double-frees without a garbage collector.

use std::cell::RefCell;
use std::rc::Rc;

/// A shared event log used to observe drop order in tests and in the demo.
pub type Log = Rc<RefCell<Vec<String>>>;

/// Simulates a resource (file handle, lock, socket) that must be released exactly once.
pub struct Guard {
    name: String,
    log: Log,
}

impl Guard {
    /// Acquire.
    pub fn acquire(name: &str, log: &Log) -> Guard {
        log.borrow_mut().push(format!("acquire {name}"));
        Guard {
            name: name.to_string(),
            log: Rc::clone(log),
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("release {}", self.name));
    }
}

/// Acquires two resources, then returns early on error. Both are still released, in reverse order.
pub fn work(fail_midway: bool, log: &Log) -> Result<(), String> {
    let _db = Guard::acquire("db", log);
    let _file = Guard::acquire("file", log);
    if fail_midway {
        return Err("something failed".into());
    }
    log.borrow_mut().push("work done".into());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn released_in_reverse_order_on_success_and_error() {
        for fail in [false, true] {
            let log: Log = Default::default();
            let _ = work(fail, &log);
            let l = log.borrow();
            assert_eq!(
                &l[l.len() - 2..],
                &["release file".to_string(), "release db".to_string()]
            );
            assert_eq!(
                l.iter().filter(|e| e.starts_with("release")).count(),
                2,
                "each exactly once"
            );
        }
    }

    #[test]
    fn released_during_panic_unwinding() {
        let log: Log = Default::default();
        let l2 = log.clone();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = Guard::acquire("lock", &l2);
            panic!("boom");
        }));
        assert!(r.is_err());
        assert_eq!(log.borrow().last().unwrap(), "release lock");
    }
}
