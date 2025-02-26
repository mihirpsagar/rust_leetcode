/// Given an undirected graph with positive weighted edges, return the minimum spanning tree using Kruskal's algorithm.
/// Taken from Page 770 of Intro to Algorithms, 4th edition.
///
/// Input: Vertices: {A, B, C, D, E, F, G, H, I}. Edges: [(A, B, 4), (A, H, 8), (B, H, 11), (H, I, 7), (B, C, 8), (H, G, 1), (I, C, 2), (C, D, 7), (C, F, 4),
///     (I, G, 6), (G, F, 2), (D, F, 14), (D, E, 9), (E, F, 10)]
/// Edge (A, B, 4) indicates an undirected edge between A and B of weight 4.
///
/// Output: 37
///
/// Data Structure used: Union Find (Disjoint Set)
///
/// Time Complexity: O(E lg(E)) (Sorting) + O(E alpha(V)) (Union Find) where alpha is the inverse Ackermann function
///
/// Algorithm (Greedy by choosing min edges):
/// 1. Sort the edges in increasing order of weights
/// 2. For every edge (u, v):
///     2.1. If find(u) != find(v), then add the edge to result
///     2.2  Union(u, v)
/// 3. Return result
///
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct DisjointSet {
    map: HashMap<char, (char, u32)>, // <Val, (Parent, Rank)>
}

impl DisjointSet {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn find(&mut self, ch: char) -> char {
        if !self.map.contains_key(&ch) {
            self.map.insert(ch, (ch, 0));
            return ch;
        }

        let &val = self.map.get(&ch).unwrap();
        if val.0 == ch {
            return ch;
        } else {
            let parent = self.find(val.0);

            // Path compression
            self.map.insert(ch, (parent, val.1));

            return parent;
        }
    }

    pub fn union(&mut self, ch1: char, ch2: char) {
        if !self.map.contains_key(&ch1) {
            self.map.insert(ch1, (ch1, 0));
        }
        if !self.map.contains_key(&ch2) {
            self.map.insert(ch2, (ch2, 0));
        }

        let parent1 = self.find(ch1);
        let parent2 = self.find(ch2);

        if parent1 == parent2 {
            return;
        }

        let rank1 = self.map.get(&parent1).unwrap().1;
        let rank2 = self.map.get(&parent2).unwrap().1;

        // Union by rank
        if rank1 < rank2 {
            self.map.insert(parent1, (parent2, rank1));
        } else if rank1 > rank2 {
            self.map.insert(parent2, (parent1, rank2));
        } else {
            self.map.insert(parent2, (parent1, rank2));
            self.map.insert(parent1, (parent1, rank1 + 1));
        }
    }

    pub fn len(&self) -> usize {
        return self.map.len();
    }
}

pub struct UndirectedGraph {
    vertices: Vec<char>,
    edges: Vec<(char, char, u32)>,
}

impl UndirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Self {
        let mut edges = edges;
        for edge in edges.iter_mut() {
            if edge.0 > edge.1 {
                let tmp = edge.0;
                edge.0 = edge.1;
                edge.1 = tmp;
            }
        }
        return Self {
            vertices: vertices,
            edges: edges,
        };
    }

    pub fn sort_edges(&mut self) {
        self.edges.sort_by(|a, b| {
            return a.2.cmp(&b.2);
        });
    }
}

pub fn krushkal_mst(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Vec<(char, char, u32)> {
    let mut result = Vec::new();
    let mut undirected_graph = UndirectedGraph::new(vertices, edges);
    let mut disjoint_set = DisjointSet::new();

    // Step 1: Sort the edges in ascending order of weights
    undirected_graph.sort_edges();

    // Step 2: For each edge, add the edge if the find doesn't match and union the disjoint sets
    for &edge in undirected_graph.edges.iter() {
        if disjoint_set.find(edge.0) != disjoint_set.find(edge.1) {
            result.push(edge.clone());
            disjoint_set.union(edge.0, edge.1);
        }
    }

    // println!("The minimmum spanning tree using Kruskal is: {:?}", result);
    return result;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let vertices = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
        let edges = vec![
            ('A', 'B', 4),
            ('A', 'H', 8),
            ('B', 'H', 11),
            ('H', 'I', 7),
            ('B', 'C', 8),
            ('H', 'G', 1),
            ('I', 'C', 2),
            ('C', 'D', 7),
            ('C', 'F', 4),
            ('I', 'G', 6),
            ('G', 'F', 2),
            ('D', 'F', 14),
            ('D', 'E', 9),
            ('E', 'F', 10),
        ];

        let result = krushkal_mst(vertices, edges);
        let mut sum = 0;
        assert_eq!(result.len(), 8);
        for &edge in result.iter() {
            sum += edge.2;
        }
        assert_eq!(sum, 37);
    }
}
