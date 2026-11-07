//! Shared ownership with `Rc`, interior mutability with `RefCell`, and `Weak` back-references,
//! which prevent the reference cycles that would otherwise leak memory.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A tree node with owned children and a non-owning (weak) link to its parent.
#[derive(Debug)]
pub struct TreeNode {
    /// Display name.
    pub name: String,
    /// Strong links down the tree.
    pub children: RefCell<Vec<Rc<TreeNode>>>,
    /// Weak link up the tree: it doesn't keep the parent alive.
    pub parent: RefCell<Weak<TreeNode>>,
}

impl TreeNode {
    /// New detached node.
    pub fn new(name: &str) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            name: name.to_string(),
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }
    /// Attaches `child` under `parent`.
    pub fn adopt(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }
    /// Walks up using weak links. Returns `None` once a parent has been dropped.
    pub fn path(node: &Rc<TreeNode>) -> String {
        let mut parts = vec![node.name.clone()];
        let mut cur = node.parent.borrow().upgrade();
        while let Some(p) = cur {
            parts.push(p.name.clone());
            cur = p.parent.borrow().upgrade();
        }
        parts.reverse();
        parts.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_parents_do_not_leak() {
        let leaf;
        {
            let root = TreeNode::new("root");
            let mid = TreeNode::new("src");
            leaf = TreeNode::new("main.rs");
            TreeNode::adopt(&root, mid.clone());
            TreeNode::adopt(&mid, leaf.clone());
            assert_eq!(TreeNode::path(&leaf), "root/src/main.rs");
            assert_eq!(
                Rc::strong_count(&root),
                1,
                "children hold only weak refs to parents"
            );
            assert_eq!(Rc::weak_count(&root), 1);
        }
        // root and mid were freed when they went out of scope, so the weak link now upgrades to None.
        assert!(leaf.parent.borrow().upgrade().is_none());
        assert_eq!(TreeNode::path(&leaf), "main.rs");
    }

    #[test]
    fn refcell_enforces_borrow_rules_at_runtime() {
        let cell = RefCell::new(vec![1]);
        let r = cell.borrow();
        assert!(
            cell.try_borrow_mut().is_err(),
            "a mutable borrow while reading is refused, not a data race"
        );
        drop(r);
        cell.borrow_mut().push(2);
        assert_eq!(*cell.borrow(), vec![1, 2]);
    }
}
