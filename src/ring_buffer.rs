//! A fixed-capacity ring buffer: no out-of-bounds writes are possible, and a full buffer is reported
//! (or the oldest item is overwritten, if you choose that) instead of silently corrupting memory.

/// Generic circular buffer backed by `Vec<Option<T>>`.
#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize, // next read
    len: usize,
}

/// Returned when pushing into a full buffer. It gives the value back so nothing is lost.
#[derive(Debug, PartialEq, Eq)]
pub struct Full<T>(pub T);

impl<T> RingBuffer<T> {
    /// Creates a buffer. Panics if `capacity == 0`, because a zero-capacity ring is a programming error.
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        RingBuffer {
            slots: (0..capacity).map(|_| None).collect(),
            head: 0,
            len: 0,
        }
    }
    /// Maximum number of elements.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// Current number of elements.
    pub fn len(&self) -> usize {
        self.len
    }
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// True if full.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }
    /// Adds to the back, or returns `Err(Full(value))` when full.
    pub fn push(&mut self, value: T) -> Result<(), Full<T>> {
        if self.is_full() {
            return Err(Full(value));
        }
        let tail = (self.head + self.len) % self.capacity();
        self.slots[tail] = Some(value);
        self.len += 1;
        Ok(())
    }
    /// Adds to the back, overwriting and returning the oldest element when full (useful for log windows).
    pub fn push_overwrite(&mut self, value: T) -> Option<T> {
        if self.is_full() {
            let old = self.slots[self.head].replace(value);
            self.head = (self.head + 1) % self.capacity();
            old
        } else {
            let _ = self.push(value);
            None
        }
    }
    /// Removes from the front.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.slots[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;
        v
    }
    /// Iterates from oldest to newest.
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        (0..self.len).filter_map(move |i| self.slots[(self.head + i) % self.capacity()].as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_with_wraparound() {
        let mut r = RingBuffer::with_capacity(3);
        r.push(1).unwrap();
        r.push(2).unwrap();
        r.push(3).unwrap();
        assert_eq!(r.push(4), Err(Full(4)));
        assert_eq!(r.pop(), Some(1));
        r.push(4).unwrap(); // wraps to slot 0
        assert_eq!(r.iter().copied().collect::<Vec<_>>(), vec![2, 3, 4]);
    }

    #[test]
    fn overwrite_mode_keeps_latest() {
        let mut r = RingBuffer::with_capacity(2);
        for i in 0..5 {
            r.push_overwrite(i);
        }
        assert_eq!(r.iter().copied().collect::<Vec<_>>(), vec![3, 4]);
    }

    #[test]
    #[should_panic(expected = "capacity must be positive")]
    fn zero_capacity_rejected() {
        let _ = RingBuffer::<u8>::with_capacity(0);
    }
}
