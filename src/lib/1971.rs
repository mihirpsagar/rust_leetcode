use std::collections::{HashMap, VecDeque};

// Time taken: 14:20, 14:32 -> Wrong, 14:35 -> Acc
struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut seen = vec![false; n as usize];

        for edge in edges {
            if let Some(list) = map.get_mut(&edge[0]) {
                list.push(edge[1]);
            } else {
                map.insert(edge[0], vec![edge[1]]);
            }
            if let Some(list) = map.get_mut(&edge[1]) {
                list.push(edge[0]);
            } else {
                map.insert(edge[1], vec![edge[0]]);
            }
        }

        queue.push_back(source);

        while let Some(node) = queue.pop_front() {
            if node == destination {
                return true;
            }

            if !seen[node as usize] {
                seen[node as usize] = true;
                if let Some(list) = map.get(&node) {
                    for &adj in list {
                        if !seen[adj as usize] {
                            queue.push_back(adj);
                        }
                    }
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
            true
        );
        assert_eq!(
            Solution::valid_path(
                6,
                vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
                0,
                5
            ),
            false
        );
        assert_eq!(
            Solution::valid_path(
                10,
                vec![
                    vec![0, 7],
                    vec![0, 8],
                    vec![6, 1],
                    vec![2, 0],
                    vec![0, 4],
                    vec![5, 8],
                    vec![4, 7],
                    vec![1, 3],
                    vec![3, 5],
                    vec![6, 5]
                ],
                7,
                5
            ),
            true
        );
    }
}
