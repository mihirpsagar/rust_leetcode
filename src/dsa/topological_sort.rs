/// Given a Directed Acyclic Graph (DAG), topological sort can be performed using Kahn's Algorithm.
/// Algorithm: Calculate in-degree for every vertex and use a queue to iterate every 0 in-degree vertex and reduce the in-degree of all connected vertices.
///
/// Input: V = 6, E = {{2,3}, {3,1}, {4,0}, {4,1}, {5,0}, {5,2}}
/// {2,3} edge indicates a directed edge from 2 -> 3 (3 is depedent on 2)
///
/// Output: 4,5,2,0,3,1
///
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};

struct Node {
    val: usize,
    // edges: Vec<Rc<RefCell<Node>>>,
    edges: Vec<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: usize) -> Self {
        return Self {
            val: val,
            edges: vec![],
        };
    }
}

struct DirectedGraph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl DirectedGraph {
    pub fn new(nodes_num: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut nodes = Vec::new();
        for idx in 0..nodes_num {
            nodes.push(Rc::new(RefCell::new(Node::new(idx))));
        }

        for (i, j) in edges {
            let node1 = nodes[i].clone();
            let node2 = nodes[j].clone();
            node1.borrow_mut().edges.push(Rc::downgrade(&node2));
        }

        return Self { nodes: nodes };
    }

    pub fn topological_sort(&mut self) -> Option<Vec<usize>> {
        let mut node_count = 0;
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut in_degree = vec![0; self.nodes.len()];

        for node in self.nodes.iter() {
            for edge in node.borrow().edges.clone() {
                if let Some(next_node) = edge.upgrade() {
                    in_degree[next_node.borrow().val] += 1;
                }
            }
        }

        for (idx, val) in in_degree.iter().enumerate() {
            if *val == 0 {
                queue.push_back(idx);
            }
        }

        while let Some(node_val) = queue.pop_front() {
            node_count += 1;
            result.push(node_val);
            for edge in self.nodes[node_val].borrow().edges.clone() {
                if let Some(next_node) = edge.upgrade() {
                    let next_node_val = next_node.borrow().val;
                    in_degree[next_node_val] -= 1;
                    if in_degree[next_node_val] == 0 {
                        queue.push_back(next_node_val);
                    }
                }
            }
        }

        if node_count == self.nodes.len() {
            return Some(result);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut directed_graph =
            DirectedGraph::new(6, vec![(2, 3), (3, 1), (4, 0), (4, 1), (5, 0), (5, 2)]);
        assert_eq!(
            directed_graph.topological_sort().unwrap(),
            vec![4, 5, 0, 2, 3, 1]
        );
        let mut directed_graph = DirectedGraph::new(5, vec![(0, 1), (1, 2), (3, 2), (3, 4)]);
        assert_eq!(
            directed_graph.topological_sort().unwrap(),
            vec![0, 3, 1, 4, 2]
        );
        let mut directed_graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 0)]);
        assert_eq!(directed_graph.topological_sort().is_none(), true);
    }
}
