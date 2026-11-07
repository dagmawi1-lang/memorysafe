//! Ownership, moves, borrowing, and lifetimes.
//!
//! **Use-after-move is a compile error** (in C++ the moved-from object is still usable and often empty):
//! ```compile_fail,E0382
//! let a = String::from("hello");
//! let b = a;             // ownership moves to `b`
//! println!("{a}");       // error[E0382]: borrow of moved value: `a`
//! ```
//! **No dangling references.** A reference cannot outlive the data it points to:
//! ```compile_fail
//! fn dangle() -> &'static String { let s = String::from("hi"); &s } // error: returns a reference to a local
//! ```
//! **Aliasing XOR mutation.** You can't hold a shared reference while mutating, which rules out iterator
//! invalidation, a classic C++ bug:
//! ```compile_fail
//! let mut v = vec![1, 2, 3];
//! let first = &v[0];
//! v.push(4);              // error[E0502]: cannot borrow `v` as mutable because it is also borrowed
//! println!("{first}");
//! ```

/// Takes ownership: the caller can no longer use `s` afterwards. The string is freed when this returns.
pub fn consume(s: String) -> usize {
    s.len()
}

/// Borrows immutably: the caller keeps ownership.
pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

/// Borrows mutably: exactly one mutable borrow can exist at a time.
pub fn shout(s: &mut String) {
    *s = s.to_uppercase();
    s.push('!');
}

/// Lifetimes: the result lives as long as the shorter of the two inputs. The compiler checks this.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Slices are bounds-checked views: no buffer over-reads. Returns `None` instead of reading past the end.
pub fn window(data: &[i32], start: usize, len: usize) -> Option<&[i32]> {
    data.get(start..start.checked_add(len)?)
}

/// Returns the first word as a borrowed slice of the input (no copy, no allocation).
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_borrow_and_mutate() {
        let s = String::from("memory");
        assert_eq!(count_vowels(&s), 2); // borrowed
        let mut t = s.clone(); // explicit deep copy
        shout(&mut t);
        assert_eq!(t, "MEMORY!");
        assert_eq!(consume(s), 6); // moved; `s` is unusable now
    }

    #[test]
    fn lifetimes_and_slices() {
        let a = String::from("ownership");
        {
            let b = String::from("borrow");
            assert_eq!(longest(&a, &b), "ownership");
        }
        let data = [1, 2, 3, 4];
        assert_eq!(window(&data, 1, 2), Some(&[2, 3][..]));
        assert_eq!(
            window(&data, 3, 5),
            None,
            "out of range is None, not undefined behaviour"
        );
        assert_eq!(
            window(&data, usize::MAX, 2),
            None,
            "overflow in start+len is caught"
        );
        assert_eq!(first_word("  safe systems  "), "safe");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexing_past_the_end_panics_instead_of_corrupting_memory() {
        let v = vec![1, 2, 3];
        let i = v.len();
        let _ = v[i];
    }
}
