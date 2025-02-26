// Time taken: 14:03, 14:13, 14:17 -> Wrong, 14:18 -> Wrong, 14:22 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue1 = VecDeque::new();
        let mut seen = HashSet::new();
        let mut visited = HashSet::new();
        let mut idx1 = 0;
        while idx1 < grid.len() {
            let mut idx2 = 0;
            while idx2 < grid[0].len() {
                if grid[idx1][idx2] == 2 {
                    queue1.push_back((idx1, idx2));
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        let mut result = -1;
        let mut queue2 = VecDeque::new();
        while !queue1.is_empty() || !queue2.is_empty() {
            if queue1.is_empty() {
                let tmp = queue1;
                queue1 = queue2;
                queue2 = tmp;
            }

            result += 1;

            while let Some(node) = queue1.pop_front() {
                let row = node.0;
                let col = node.1;

                // Base case
                if !visited.insert((row, col)) {
                    continue;
                }
                if grid[row][col] == 0 {
                    continue;
                }

                // Update to rotten
                grid[row][col] = 2;

                // Right
                if col + 1 < grid[0].len() && grid[row][col + 1] == 1 {
                    if seen.insert((row, col + 1)) {
                        queue2.push_back((row, col + 1));
                    }
                }

                // Bottom
                if row + 1 < grid.len() && grid[row + 1][col] == 1 {
                    if seen.insert((row + 1, col)) {
                        queue2.push_back((row + 1, col));
                    }
                }

                // Left
                if col != 0 && grid[row][col - 1] == 1 {
                    if seen.insert((row, col - 1)) {
                        queue2.push_back((row, col - 1));
                    }
                }

                // Top
                if row != 0 && grid[row - 1][col] == 1 {
                    if seen.insert((row - 1, col)) {
                        queue2.push_back((row - 1, col));
                    }
                }
            }
        }

        idx1 = 0;
        while idx1 < grid.len() {
            let mut idx2 = 0;
            while idx2 < grid[0].len() {
                if grid[idx1][idx2] == 1 {
                    return -1;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return std::cmp::max(0, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
        assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0);
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 2], vec![1, 1], vec![0, 0], vec![2, 0]]),
            1
        );
    }
}
