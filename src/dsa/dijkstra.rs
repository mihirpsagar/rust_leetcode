/// Given a directed graph with non -ve edges and a source vertex, return the shortest paths to all other vertices from the source vertex using Dijktra's algorithm.
/// Taken from Page 805 of Intro to Algorithms, 4th edition.
///
/// Time complexity: O(V * lg(V)) // Min Heap Initialization + O((V + E) * lg(V)) // Iteration (Worst case fully connected graph E = V^2 - V)
///
/// Data structure used: Min Heap with update node
///
/// Algorithm:
/// 1. Initialize min heap with all vertices with infinite distance except source vertex with 0 distance.
/// 2. Initialize parent for all vertices to be null. This will be used to formulate the resulting path.
/// 3. Keep popping the smallest vertex from min heap and perform the following:
/// 3.1. Iterate through the adjacent edges and relax the edge and update the vertex in min heap.
/// 3.2. Update the parent when the min heap vertex is updated.
/// relax edge (u, v, w) -> if dist(u) + weight(u, v) < dist(v) then update dist(v) and parent of v = u
/// 4. Use the parent and distance map to produce the shortest path.
use std::collections::HashMap;

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

    pub fn pop(&mut self) -> Option<(u32, char)> {
        if self.len == 0 {
            return None;
        }

        let result = self.arr[0].clone();
        self.swap(0, self.len - 1);
        self.len -= 1;

        self.heapify(0);
        self.map.remove(&result.1);

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

pub struct DirectedGraph {
    map: HashMap<char, Vec<(char, u32)>>, // <source vertex, Vec<target vertex, weight>>
}

impl DirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Self {
        let mut map = HashMap::new();

        for vertex in vertices {
            map.insert(vertex, Vec::new());
        }

        for edge in edges {
            if let Some(adj_list) = map.get_mut(&edge.0) {
                adj_list.push((edge.1, edge.2));
            }
        }

        return Self { map: map };
    }
}

pub fn dijkstra(
    vertices: Vec<char>,
    edges: Vec<(char, char, u32)>,
    source: char,
) -> Vec<(char, char, u32)> {
    let mut result = Vec::new();
    let directed_graph = DirectedGraph::new(vertices.clone(), edges);

    // Step 1: Initialize min heap with all vertices inf distance except source
    let mut min_heap = MinHeap::new();
    for &vertex in vertices.iter() {
        if vertex != source {
            min_heap.push((u32::MAX, vertex));
        }
    }
    min_heap.push((0, source));

    // Step 2: Initialize parent and distance map for all vertices
    let mut dist_parent_map: HashMap<char, (u32, Option<char>)> = HashMap::new();
    for &vertex in vertices.iter() {
        if vertex != source {
            dist_parent_map.insert(vertex, (u32::MAX, None));
        }
    }
    dist_parent_map.insert(source, (0, None));

    // Step 3: Iterate through the min heap and adjacent edges
    while let Some(node) = min_heap.pop() {
        for &adj_edge in directed_graph.map.get(&node.1).unwrap().iter() {
            let dist_v = dist_parent_map.get(&adj_edge.0).unwrap().0;
            if node.0.saturating_add(adj_edge.1) < dist_v {
                dist_parent_map.insert(adj_edge.0, (node.0 + adj_edge.1, Some(node.1)));
                min_heap.update(adj_edge.0, node.0 + adj_edge.1);
            }
        }
    }

    // Step 4: Create the shortest path result
    for (key, val) in dist_parent_map {
        if let Some(parent) = val.1 {
            result.push((parent, key, val.0));
        }
    }

    return result;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let vertices = vec!['S', 'T', 'X', 'Y', 'Z'];
        let edges = vec![
            ('S', 'T', 10),
            ('S', 'Y', 5),
            ('T', 'Y', 2),
            ('T', 'X', 1),
            ('X', 'Z', 4),
            ('Y', 'T', 3),
            ('Y', 'X', 9),
            ('Y', 'Z', 2),
            ('Z', 'S', 7),
            ('Z', 'X', 6),
        ];
        let result = dijkstra(vertices, edges, 'S');
        assert_eq!(result.contains(&('S', 'Y', 5)), true);
        assert_eq!(result.contains(&('Y', 'Z', 7)), true);
        assert_eq!(result.contains(&('Y', 'T', 8)), true);
        assert_eq!(result.contains(&('T', 'X', 9)), true);
    }
}
