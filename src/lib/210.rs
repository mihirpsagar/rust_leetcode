// Time taken: 22:05, 22:15 -> Acc

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut node_count = 0;
        let mut in_degree = vec![0; num_courses as usize];
        let mut queue = VecDeque::new();
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for val in prerequisites {
            if let Some(arr) = map.get_mut(&val[1]) {
                arr.push(val[0]);
            } else {
                map.insert(val[1], vec![val[0]]);
            }
            in_degree[val[0] as usize] += 1;
        }

        for (idx, &val) in in_degree.iter().enumerate() {
            if val == 0 {
                queue.push_back(idx as i32);
            }
        }

        while let Some(node) = queue.pop_front() {
            result.push(node);
            node_count += 1;
            if let Some(dep) = map.get(&node) {
                for &dep_node in dep {
                    in_degree[dep_node as usize] -= 1;
                    if in_degree[dep_node as usize] == 0 {
                        queue.push_back(dep_node);
                    }
                }
            }
        }

        if node_count == num_courses {
            return result;
        } else {
            return Vec::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), [0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            [0, 1, 2, 3]
        );
        assert_eq!(Solution::find_order(1, vec![]), [0]);
    }
}
