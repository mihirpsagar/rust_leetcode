/// Given an undirected graph with positive weighted edges, return the minimum spanning tree using Prim's algorithm.
/// Taken from Page 775 of Intro to Algorithms, 4th edition.
///
/// Input: Vertices: {A, B, C, D, E, F, G, H, I}. Edges: [(A, B, 4), (A, H, 8), (B, H, 11), (H, I, 7), (B, C, 8), (H, G, 1), (I, C, 2), (C, D, 7), (C, F, 4),
///     (I, G, 6), (G, F, 2), (D, F, 14), (D, E, 9), (E, F, 10)]
/// Edge (A, B, 4) indicates an undirected edge between A and B of weight 4.
///
/// Output: 37
///
/// Data Structure used: Min Heap (Min Priority Queue) + HashSet (Visited nodes)
///
/// Time Complexity: O(V lg(V)) (Min Heap) + O(E lg(V)) (Iterate through adjacent edges and reduce weight for vertices in heap)
///
/// Algorithm (Greedy by choosing a vertex and then choosing the min edges to unvisited vertex):
/// 1. Add all the vertices to min heap with weight infinity except the starting vertex with weight 0.
/// 2. While not all vertices are visited:
/// 2.1. Pop the minimum vertex from heap and add to visited set.
/// 2.2. For every adjacent edge, reduce the weight of adjacent vertex if weight is lesser than the weight in heap.
/// 2.3. Store the parent vertex for every vertex weight reduced which can then be used to construct the MST.
///
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

// Custom implementation of Min Heap as std::collections::BinaryHeap doesn't support update.
pub struct MinHeap {
    arr: Vec<(u32, char)>,     // (weight, vertex)
    map: HashMap<char, usize>, // Used to find the position of the vertex in O(1). Used for update().
    len: usize,
}

impl MinHeap {
    pub fn new() -> Self {
        return Self {
            arr: Vec::new(),
            map: HashMap::new(),
            len: 0,
        };
    }

    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        if idx1 == idx2 || idx1 >= self.len || idx2 >= self.len {
            return;
        }

        self.map.insert(self.arr[idx1].1, idx2);
        self.map.insert(self.arr[idx2].1, idx1);

        let tmp = self.arr[idx1];
        self.arr[idx1] = self.arr[idx2];
        self.arr[idx2] = tmp;
    }

    pub fn percolate_up(&mut self, idx: usize) {
        if idx == 0 || idx >= self.len {
            return;
        }

        let parent = (idx - 1) / 2;

        if self.arr[parent].0 > self.arr[idx].0 {
            self.swap(parent, idx);
            self.percolate_up(parent);
        }
    }

    pub fn heapify(&mut self, idx: usize) {
        if idx >= self.len {
            return;
        }

        let left = (idx * 2) + 1;
        let right = (idx * 2) + 1;
        let mut smallest = idx;

        if left < self.len && self.arr[left].0 < self.arr[smallest].0 {
            smallest = left;
        }

        if right < self.len && self.arr[right].0 < self.arr[smallest].0 {
            smallest = right;
        }

        if smallest != idx {
            self.swap(idx, smallest);
            self.heapify(smallest);
        }
    }

    pub fn push(&mut self, val: (u32, char)) {
        // If length is equal to arr size, push and percolate up
        if self.len == self.arr.len() {
            self.arr.push(val);
        } else {
            self.arr[self.len] = val;
        }

        self.map.insert(val.1, self.len);
        self.len += 1;

        self.percolate_up(self.len - 1);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.len == 0 {
            return None;
        }

        let result = self.arr[0].1.clone();
        self.swap(0, self.len - 1);
        self.len -= 1;

        self.heapify(0);
        self.map.remove(&result);

        return Some(result);
    }

    pub fn update(&mut self, node: char, val: u32) {
        if let Some(&idx) = self.map.get(&node) {
            if idx >= self.len || self.arr[idx].1 != node {
                panic!("Min heap map implementation has gone wrong!");
            }
            self.arr[idx].0 = val;

            // Ensure bottom tree is valid and then percolate up.
            self.heapify(idx);
            self.percolate_up(idx);
        }
    }

    pub fn get(&self, node: char) -> Option<(u32, char)> {
        if let Some(&idx) = self.map.get(&node) {
            if idx >= self.len || self.arr[idx].1 != node {
                return None;
            } else {
                return Some(self.arr[idx]);
            }
        } else {
            return None;
        }
    }
}

pub struct UndirectedGraph {
    map: HashMap<char, Vec<(char, u32)>>, // HashMap<Source Vertex, Vec<(Target Vertex, edge)>>
}

impl UndirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Self {
        let mut map = HashMap::new();

        for vertex in vertices {
            map.insert(vertex, Vec::new());
        }

        for edge in edges {
            let adj_edges = map.get_mut(&edge.0).unwrap();
            adj_edges.push((edge.1, edge.2));
            let adj_edges = map.get_mut(&edge.1).unwrap();
            adj_edges.push((edge.0, edge.2));
        }

        return Self { map: map };
    }
}

pub fn prim_mst(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Vec<(char, char, u32)> {
    let mut result = Vec::new();
    let undirected_graph = UndirectedGraph::new(vertices.clone(), edges);
    let mut parent_map = HashMap::new();

    // Step 1: Add all vertices to min heap and set the value as inf except one.
    // let mut min_heap: BinaryHeap<Reverse<(u32, char)>> = BinaryHeap::new();
    // min_heap.push(Reverse((0, vertices[0])));
    let mut min_heap = MinHeap::new();
    min_heap.push((0, vertices[0]));

    for &vertex in vertices.iter().skip(1) {
        // min_heap.push(Reverse((u32::MAX, vertex)));
        min_heap.push((u32::MAX, vertex));
    }

    // Step 2: Pop the minimum heap vertex and update the weights of other vertices.
    while let Some(node) = min_heap.pop() {
        for &edge in undirected_graph.map.get(&node).unwrap().iter() {
            if let Some(heap_val) = min_heap.get(edge.0) {
                if edge.1 < heap_val.0 {
                    min_heap.update(edge.0, edge.1);
                    parent_map.insert(edge.0, node);
                }
            }
        }
    }

    // Step 3: Form resulting MST using the parent map
    for (key, val) in parent_map {
        'inner: for &edge in undirected_graph.map.get(&key).unwrap().iter() {
            if edge.0 == val {
                result.push((key, val, edge.1));
                break 'inner;
            }
        }
    }

    // println!("The resulting MST using Prim's is: {:?}", result);
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

        let result = prim_mst(vertices, edges);
        let mut sum = 0;
        assert_eq!(result.len(), 8);
        for &edge in result.iter() {
            sum += edge.2;
        }
        assert_eq!(sum, 37);
    }
}
