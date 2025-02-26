/// Given a directed graph, return a list of strongly connected components.
/// Taken from Page 752 of Intro to Algorithms, 4th edition.
///
/// Input: Vertices: {A, B, C, D, E, F, G, H}. Edges: [(A, B), (B, E), (E, A), (B, F), (F, G), (G, F), (C, D), (D, C), (C, G), (D, H), (G, H), (H, H)]
/// Edge (A, B) indicates directed edge from A to B.
///
/// Output: [(A,B,E), (C, D), (F, G), (H)]
///
/// Algorithm:
/// 1. Call DFS on Graph G to compute the finish times for each vertex.
/// 2. Create Graph G' (Transpose) which has the directed edges reversed.
/// 3. Call DFS on G' in decreasing order of finish times from G.
/// 4. Output the DFS traversal of G' as the list of strongly connected components for G.
///
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node {
    val: char,
    edges: Vec<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: char) -> Self {
        return Self {
            val: val,
            edges: Vec::new(),
        };
    }
}

struct DirectedGraph {
    nodes: HashMap<char, Rc<RefCell<Node>>>,
}

impl DirectedGraph {
    pub fn new(vertices: Vec<char>, edges: Vec<(char, char)>) -> Self {
        let mut nodes = HashMap::new();
        for ch in vertices {
            nodes.insert(ch, Rc::new(RefCell::new(Node::new(ch))));
        }

        for edge in edges {
            let node1 = nodes.get(&edge.0).unwrap();
            node1
                .borrow_mut()
                .edges
                .push(Rc::downgrade(nodes.get(&edge.1).unwrap()));
        }

        return Self { nodes: nodes };
    }

    pub fn new_transpose(vertices: Vec<char>, edges: Vec<(char, char)>) -> Self {
        let mut nodes = HashMap::new();
        for ch in vertices {
            nodes.insert(ch, Rc::new(RefCell::new(Node::new(ch))));
        }

        for edge in edges {
            let node1 = nodes.get(&edge.1).unwrap();
            node1
                .borrow_mut()
                .edges
                .push(Rc::downgrade(nodes.get(&edge.0).unwrap()));
        }

        return Self { nodes: nodes };
    }

    pub fn get_nodes_iter(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut result = Vec::new();
        for val in self.nodes.values() {
            result.push(val.clone());
        }

        return result;
    }
}

// enum NodeVisitState {
//     White, // Not visited
//     Gray,  // Visited but not completed
//     Black, // Completely visited
// }

pub fn strongly_connected_components_directed_graph(
    vertices: Vec<char>,
    edges: Vec<(char, char)>,
) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let graph = DirectedGraph::new(vertices.clone(), edges.clone());

    // Step 1: Call DFS on graph and compute finish times for each vertex
    let mut node_time: HashMap<char, (u32, u32)> = HashMap::new();
    // let mut visited: HashMap<char, NodeVisitState> = HashMap::new();
    let mut visited: HashMap<char, bool> = HashMap::new();
    let graph_nodes = graph.get_nodes_iter();
    let mut time: u32 = 0;

    for node in graph_nodes.iter() {
        let ch = node.borrow().val;
        node_time.insert(ch, (0, 0));
        visited.insert(ch, false);
    }

    for node in graph_nodes.iter() {
        dfs(node.clone(), &mut node_time, &mut visited, &mut time);
    }

    // println!("{:?}", node_time);

    // Step 2: Create G' transpose which has the edges reversed.
    let graph_transpose = DirectedGraph::new_transpose(vertices, edges);

    // Step 3: Call DFS on G' in decreasing order of finish times.
    for node in graph_nodes.iter() {
        let ch = node.borrow().val;
        visited.insert(ch, false);
    }

    let mut ordered_node_time = Vec::new();

    for (key, val) in node_time {
        binary_insert(&mut ordered_node_time, key, val.1);
    }

    for &val in ordered_node_time.iter().rev() {
        let mut curr_result = Vec::new();
        dfs_result(
            graph_transpose.nodes.get(&val.0).unwrap().clone(),
            &mut visited,
            &mut curr_result,
        );
        if !curr_result.is_empty() {
            result.push(curr_result);
        }
    }

    // println!(
    //     "The strongly connected components in the directed graph are: {:?}",
    //     result
    // );
    return result;
}

pub fn binary_insert(arr: &mut Vec<(char, u32)>, key: char, val: u32) {
    if arr.is_empty() {
        arr.push((key, val));
        return;
    }

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid].1 < val {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    arr.insert(left, (key, val));
}

pub fn dfs(
    node: Rc<RefCell<Node>>,
    mut node_time: &mut HashMap<char, (u32, u32)>,
    mut visited: &mut HashMap<char, bool>,
    mut time: &mut u32,
) {
    let node = node.borrow();
    let ch = node.val;

    // Base case
    if *visited.get(&ch).unwrap() {
        return;
    }

    *time += 1;
    let start_time = *time;
    // println!("Visiting node: {} at time {}", ch, *time);
    visited.insert(ch, true);
    node_time.insert(ch, (*time, u32::MAX));

    for edge in node.edges.iter() {
        let upgraded_edge = edge.upgrade().unwrap();
        let adj_node = upgraded_edge.borrow();
        if !*visited.get(&adj_node.val).unwrap() {
            dfs(
                upgraded_edge.clone(),
                &mut node_time,
                &mut visited,
                &mut time,
            );
        }
    }

    *time += 1;
    // println!("Inserting time {} to node {}", *time, ch);
    node_time.insert(ch, (start_time, *time));
}

pub fn dfs_result(
    node: Rc<RefCell<Node>>,
    mut visited: &mut HashMap<char, bool>,
    mut result: &mut Vec<char>,
) {
    let node = node.borrow();

    // Base case
    if *visited.get(&node.val).unwrap() == true {
        return;
    }

    visited.insert(node.val, true);
    result.push(node.val);

    for edge in node.edges.iter() {
        let upgraded_edge = edge.upgrade().unwrap();
        let adj_node = upgraded_edge.borrow();
        if !*visited.get(&adj_node.val).unwrap() {
            dfs_result(upgraded_edge.clone(), &mut visited, &mut result);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let vertices = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let edges = vec![
            ('A', 'B'),
            ('B', 'E'),
            ('E', 'A'),
            ('B', 'F'),
            ('F', 'G'),
            ('G', 'F'),
            ('C', 'D'),
            ('D', 'C'),
            ('C', 'G'),
            ('D', 'H'),
            ('G', 'H'),
            ('H', 'H'),
        ];
        let mut result = strongly_connected_components_directed_graph(vertices, edges);
        assert_eq!(result.len(), 4);
        let mut idx = 0;
        while idx < result.len() {
            let mut res = result[idx].clone();
            res.sort();
            result[idx] = res;
            idx += 1;
        }
        result.sort_by(|a, b| {
            return a[0].cmp(&b[0]);
        });
        assert_eq!(
            result,
            vec![
                vec!['A', 'B', 'E'],
                vec!['C', 'D'],
                vec!['F', 'G'],
                vec!['H']
            ]
        );
    }
}
