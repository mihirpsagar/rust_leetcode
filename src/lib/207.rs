// Time taken: 15:55, 16:19 -> Acc, 16:50 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

// Solution 2: Topological Sort using Kahn's Algorithm
// Time complexity: O(V + E)
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_degree = HashMap::new();
        let mut adj_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for node in 0..num_courses {
            in_degree.insert(node, 0);
        }
        for row in prerequisites {
            *in_degree.entry(row[0]).or_insert(0) += 1;
            if let Some(arr) = adj_map.get_mut(&row[1]) {
                arr.push(row[0]);
            } else {
                adj_map.insert(row[1], vec![row[0]]);
            }
        }

        let mut queue = VecDeque::new();
        let mut topological_order = Vec::new();

        for (&key, &val) in in_degree.iter() {
            if val == 0 {
                queue.push_back(key);
            }
        }

        while let Some(node) = queue.pop_front() {
            topological_order.push(node);
            if let Some(adj_arr) = adj_map.get(&node) {
                for val in adj_arr {
                    let prev_val = in_degree.get(val).unwrap().clone();
                    in_degree.insert(*val, prev_val - 1);
                    if prev_val - 1 == 0 {
                        queue.push_back(*val);
                    }
                }
            }
        }

        return topological_order.len() == num_courses as usize;
    }
}

// Solution 1: DFS.
// Time complexity: O(V + E)
// impl Solution {
//     pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//         let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
//         for row in prerequisites {
//             if let Some(arr) = map.get_mut(&row[0]) {
//                 arr.push(row[1]);
//             } else {
//                 map.insert(row[0], vec![row[1]]);
//             }
//         }

//         let mut visited = HashSet::new();
//         for node in 0..num_courses {
//             if !Self::dfs(node, &mut map, &mut visited) {
//                 return false;
//             }
//         }

//         return true;
//     }

//     fn dfs(
//         node: i32,
//         mut map: &mut HashMap<i32, Vec<i32>>,
//         mut visited: &mut HashSet<i32>,
//     ) -> bool {
//         // Base case
//         if visited.contains(&node) {
//             return false;
//         }
//         if !map.contains_key(&node) {
//             return true;
//         }

//         let dependency_arr = map.get(&node).unwrap().clone();

//         visited.insert(node);
//         for val in dependency_arr {
//             if !Self::dfs(val, &mut map, &mut visited) {
//                 return false;
//             }
//         }
//         visited.remove(&node);

//         map.remove(&node);

//         return true;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
}
