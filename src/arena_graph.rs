//! Graphs without pointers. Cyclic pointer graphs are a common source of leaks and dangling pointers in C/C++.
//! Here nodes live in a `Vec` (an arena) and edges are typed indices, so there is nothing to dangle.

use std::collections::VecDeque;

/// A handle to a node. A newtype prevents mixing it up with ordinary integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

/// A directed graph that owns all of its nodes.
#[derive(Debug, Default)]
pub struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<Vec<NodeId>>,
}

impl<T> Graph<T> {
    /// Empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    /// Adds a node and returns its handle.
    pub fn add(&mut self, value: T) -> NodeId {
        self.nodes.push(value);
        self.edges.push(Vec::new());
        NodeId(self.nodes.len() - 1)
    }
    /// Adds a directed edge. Cycles are fine: there is no reference counting to leak.
    pub fn connect(&mut self, from: NodeId, to: NodeId) {
        self.edges[from.0].push(to);
    }
    /// Borrows a node's value.
    pub fn get(&self, id: NodeId) -> &T {
        &self.nodes[id.0]
    }
    /// Breadth-first shortest path (by hop count), if one exists.
    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        let mut prev = vec![None; self.nodes.len()];
        let mut seen = vec![false; self.nodes.len()];
        let mut q = VecDeque::from([from]);
        seen[from.0] = true;
        while let Some(n) = q.pop_front() {
            if n == to {
                let mut path = vec![to];
                let mut cur = to;
                while let Some(p) = prev[cur.0] {
                    path.push(p);
                    cur = p;
                }
                path.reverse();
                return Some(path);
            }
            for &m in &self.edges[n.0] {
                if !seen[m.0] {
                    seen[m.0] = true;
                    prev[m.0] = Some(n);
                    q.push_back(m);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_graph_shortest_path() {
        let mut g = Graph::new();
        let [a, b, c, d] = ["router", "switch", "server", "laptop"].map(|n| g.add(n));
        g.connect(a, b);
        g.connect(b, c);
        g.connect(c, a); // cycle
        g.connect(b, d);
        let path = g.shortest_path(a, d).unwrap();
        assert_eq!(
            path.iter().map(|&n| *g.get(n)).collect::<Vec<_>>(),
            vec!["router", "switch", "laptop"]
        );
        assert_eq!(g.shortest_path(d, a), None);
    }
}
