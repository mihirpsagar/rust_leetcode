/// Given a directed graph with negative weights, find the shortest path from one vertex to all the other vertices using Bellman Ford algorithm.
/// If the graph contains a negative cycle, then return false.
/// Taken from Page 795 of Intro to Algorithms, 4th edition.
///
/// Time complexity: O(V * (V + E)) = O(V^2 + (V * E))
///
/// Algorithm:
/// 1. For each vertex except source, initialize distance to be infinity and parent to be null.
/// 2. Create a list of all edges which can be used to loop easily.
/// 3. For 0..n-1:
/// 3.1. For each edge in list:
/// 3.1.1. relax edge (u, v, w) -> if dist(u) + weight(u, v) < dist(v) then update dist(v) and parent of v = u
/// 3.1.2. If no edge is relaxed, break loop 3.
/// 4. For each edge, check if relax is possible (If yes, then return false since it has -ve cycle)
/// 5. Use the parent to create the resulting path.
use std::collections::HashMap;

pub struct DirectedGraph {
    map: HashMap<char, Vec<(char, i32)>>, // <source vertex, Vec<target vertex, weight>>
}

impl DirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char, i32)>) -> Self {
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

    pub fn get_edge_list(&self) -> Vec<(char, char, i32)> {
        let mut edge_list = Vec::new();

        for (&key, ref val) in self.map.iter() {
            for &edge in val.iter() {
                edge_list.push((key, edge.0, edge.1));
            }
        }

        return edge_list;
    }
}

pub fn bellman_ford(
    vertices: Vec<char>,
    edges: Vec<(char, char, i32)>,
    source_vertex: char,
) -> Option<Vec<(char, char, i32)>> {
    let mut result = Vec::new();
    // let directed_graph = DirectedGraph::new(vertices.clone(), edges.clone());

    // Step 1: Create distance and parent map
    let mut dist_parent_map: HashMap<char, (i32, Option<char>)> = HashMap::new();
    for &vertex in vertices.iter() {
        dist_parent_map.insert(vertex, (i32::MAX, None));
    }
    dist_parent_map.insert(source_vertex, (0, None));

    // Step 2: Relax all edges (V -1) times
    for _ in 0..(vertices.len() - 1) {
        let mut edge_updated = false;
        for &edge in edges.iter() {
            let dist_u = dist_parent_map.get(&edge.0).unwrap().0;
            let dist_v = dist_parent_map.get(&edge.1).unwrap().0;
            if dist_v > dist_u.saturating_add(edge.2) {
                edge_updated = true;
                dist_parent_map.insert(edge.1, (dist_u + edge.2, Some(edge.0)));
            }
        }

        // If no edge is updated, then break
        if !edge_updated {
            break;
        }
    }

    // Step 3: Check if -ve cycle is present
    for &edge in edges.iter() {
        let dist_u = dist_parent_map.get(&edge.0).unwrap().0;
        let dist_v = dist_parent_map.get(&edge.1).unwrap().0;
        if dist_v > dist_u.saturating_add(edge.2) {
            return None;
        }
    }

    // Step 4: Create the resulting path from source to all vertices
    for (key, val) in dist_parent_map {
        if let Some(parent) = val.1 {
            result.push((parent, key, val.0));
        }
    }

    return Some(result);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let vertices = vec!['S', 'T', 'X', 'Y', 'Z'];
        let edges = vec![
            ('S', 'T', 6),
            ('S', 'Y', 7),
            ('T', 'Y', 8),
            ('T', 'X', 5),
            ('T', 'Z', -4),
            ('X', 'T', -2),
            ('Y', 'X', -3),
            ('Y', 'Z', 9),
            ('Z', 'X', 7),
            ('Z', 'S', 2),
        ];
        let result = bellman_ford(vertices, edges, 'S');
        assert_eq!(result.is_some(), true);
        let result = result.unwrap();
        assert_eq!(result.contains(&('S', 'Y', 7)), true);
        assert_eq!(result.contains(&('Y', 'X', 4)), true);
        assert_eq!(result.contains(&('X', 'T', 2)), true);
        assert_eq!(result.contains(&('T', 'Z', -2)), true);

        let vertices = vec!['A', 'B', 'C', 'D'];
        let edges = vec![
            ('A', 'B', 5),
            ('A', 'D', 4),
            ('B', 'C', 3),
            ('C', 'D', -10),
            ('D', 'B', 5),
        ];
        let result = bellman_ford(vertices, edges, 'A');
        assert_eq!(result.is_none(), true);
    }
}
