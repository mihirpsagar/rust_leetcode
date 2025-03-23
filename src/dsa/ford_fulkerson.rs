/// Given a directed graph with a source vertex and sink / target vertex, find the maximum flow using Ford-Fulkerson algorithm.
/// Adjacency matrix is used to represent the graph as it's easier to implement.
/// The graph is modified to make it a residual graph by adding back flows to allow undo operations.
/// DFS is used to find a path from source to target.
///
/// Time Complexity: O((V + E) * max_flow) // DFS * max_flow
///
use std::collections::{HashMap, HashSet, VecDeque};

pub struct DirectedGraph {
    vertex_position: HashMap<char, usize>,
    position_vertex: HashMap<usize, char>,
    edges: Vec<Vec<u32>>,
    len: usize,
}

impl DirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char, u32)>) -> Self {
        let n = vertices.len();
        let mut vertex_position = HashMap::new();
        let mut position_vertex = HashMap::new();
        let mut edges_matrix = Vec::new();

        for (idx, &vertex) in vertices.iter().enumerate() {
            vertex_position.insert(vertex, idx);
            position_vertex.insert(idx, vertex);
        }

        for _ in 0..n {
            edges_matrix.push(vec![0; n]);
        }

        for (v1, v2, val) in edges {
            let idx1 = *vertex_position.get(&v1).unwrap();
            let idx2 = *vertex_position.get(&v2).unwrap();
            edges_matrix[idx1][idx2] = val;
        }

        return Self {
            vertex_position: vertex_position,
            position_vertex: position_vertex,
            edges: edges_matrix,
            len: n,
        };
    }

    pub fn get_adj_vertices(&self, vertex: char) -> Vec<char> {
        let mut result = Vec::new();
        let idx1 = *self.vertex_position.get(&vertex).unwrap();
        for idx2 in 0..self.len {
            if self.edges[idx1][idx2] > 0 {
                result.push(*self.position_vertex.get(&idx2).unwrap());
            }
        }
        return result;
    }
}

pub fn ford_fulkerson(
    vertices: Vec<char>,
    edges: Vec<(char, char, u32)>,
    source_vertex: char,
    target_vertex: char,
) -> u32 {
    let mut result = 0;

    let mut directed_graph = DirectedGraph::new(vertices, edges);
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    // Loop until there is no path from Source to Target
    loop {
        // Step 1: DFS to find a path from source to vertex.
        visited.clear();
        path.clear();

        // If no path exists, max flow is found.
        if !dfs(
            &directed_graph,
            source_vertex,
            target_vertex,
            &mut visited,
            &mut path,
        ) {
            break;
        }

        // Step 2: Add the minimum flow in the path and update the graph to make it residual (back flow)
        result += add_min_flow(&mut directed_graph, &path);
    }

    return result;
}

pub fn dfs(
    directed_graph: &DirectedGraph,
    curr_vertex: char,
    target_vertex: char,
    mut visited: &mut HashSet<char>,
    mut path: &mut Vec<char>,
) -> bool {
    if visited.contains(&curr_vertex) {
        return false;
    }

    visited.insert(curr_vertex);
    path.push(curr_vertex);

    if curr_vertex == target_vertex {
        return true;
    }

    let adj_vertices = directed_graph.get_adj_vertices(curr_vertex);
    for v in adj_vertices {
        if dfs(&directed_graph, v, target_vertex, &mut visited, &mut path) {
            return true;
        }
    }

    path.pop();
    return false;
}

pub fn add_min_flow(directed_graph: &mut DirectedGraph, path: &Vec<char>) -> u32 {
    // Step 1: Calculate minimum flow possible in the path.
    let mut min = u32::MAX;

    let mut idx = 0;
    while idx < path.len() - 1 {
        let pos1 = *directed_graph.vertex_position.get(&path[idx]).unwrap();
        let pos2 = *directed_graph.vertex_position.get(&path[idx + 1]).unwrap();
        min = std::cmp::min(min, directed_graph.edges[pos1][pos2]);

        idx += 1;
    }

    assert!(min > 0, "Trying to add flow when there's no capacity.");

    // Step 2: Reduce the minimum from A -> B and add the minimum from B -> A
    idx = 0;
    while idx < path.len() - 1 {
        let pos1 = *directed_graph.vertex_position.get(&path[idx]).unwrap();
        let pos2 = *directed_graph.vertex_position.get(&path[idx + 1]).unwrap();
        directed_graph.edges[pos1][pos2] -= min;
        directed_graph.edges[pos2][pos1] += min;

        idx += 1;
    }

    return min;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut result = ford_fulkerson(
            vec!['S', 'A', 'B', 'C', 'D', 'T'],
            vec![
                ('S', 'A', 10),
                ('S', 'B', 10),
                ('A', 'B', 2),
                ('A', 'C', 4),
                ('A', 'D', 8),
                ('B', 'D', 9),
                ('D', 'C', 6),
                ('C', 'T', 10),
                ('D', 'T', 10),
            ],
            'S',
            'T',
        );

        assert_eq!(result, 19);

        result = ford_fulkerson(
            vec!['S', 'A', 'B', 'C', 'D', 'T'],
            vec![
                ('S', 'A', 16),
                ('S', 'B', 13),
                ('A', 'B', 10),
                ('A', 'C', 12),
                ('B', 'A', 4),
                ('B', 'D', 14),
                ('C', 'B', 9),
                ('C', 'T', 20),
                ('D', 'C', 7),
                ('D', 'T', 4),
            ],
            'S',
            'T',
        );

        assert_eq!(result, 23);
    }
}
