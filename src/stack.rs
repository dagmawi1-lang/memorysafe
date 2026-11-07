//! A singly linked stack built from `Option<Box<Node<T>>>`: no null pointers, no manual `free`,
//! and no way to double-free or use a node after popping it.

/// A LIFO stack.
#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Stack { head: None, len: 0 }
    }
    /// Pushes a value. `take()` moves the old head out, leaving `None` behind, so ownership is always clear.
    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.len += 1;
    }
    /// Pops the top value. The node's memory is freed automatically when the `Box` goes out of scope.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }
    /// Borrows the top value.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.value)
    }
    /// Mutably borrows the top value.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|n| &mut n.value)
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.len
    }
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Borrowing iterator from top to bottom.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

/// Iterator over `&T`. The lifetime ties it to the stack, so the stack can't be modified while you iterate.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.value
        })
    }
}

/// Owning iterator.
pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/// The default recursive drop would recurse once per node and overflow the call stack on very long lists.
/// Dropping iteratively keeps it O(1) stack space.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_peek() {
        let mut s = Stack::new();
        assert_eq!(s.pop(), None);
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        if let Some(top) = s.peek_mut() {
            *top = 30;
        }
        assert_eq!(s.iter().copied().collect::<Vec<_>>(), vec![30, 2, 1]);
        assert_eq!(s.pop(), Some(30));
        assert_eq!(s.len(), 2);
        assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![2, 1]);
    }

    #[test]
    fn million_nodes_drop_without_stack_overflow() {
        let mut s = Stack::new();
        for i in 0..1_000_000 {
            s.push(i);
        }
        assert_eq!(s.len(), 1_000_000);
        drop(s);
    }

    #[test]
    fn owns_heap_values() {
        let mut s = Stack::new();
        s.push(String::from("a"));
        s.push(String::from("b"));
        let b = s.pop().unwrap();
        assert_eq!(b, "b"); // we own the popped String; the node is already freed
    }
}
