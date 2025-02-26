// Time taken: 13:10, 13:42 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn islandsAndTreasure(grid: &mut Vec<Vec<i32>>) {
        let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut idx1 = 0;
        while idx1 < grid.len() {
            let mut idx2 = 0;
            while idx2 < grid[0].len() {
                if grid[idx1][idx2] == 0 {
                    queue.push_back(((idx1, idx2), 0));
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        while let Some(node) = queue.pop_front() {
            let row = node.0 .0;
            let col = node.0 .1;
            let dist = node.1;

            // Base case
            if !visited.insert((row, col)) {
                continue;
            }
            if grid[row][col] == -1 {
                continue;
            }

            grid[row][col] = std::cmp::min(grid[row][col], dist);

            // Right
            if col + 1 < grid[0].len() {
                queue.push_back(((row, col + 1), dist + 1));
            }

            // Bottom
            if row + 1 < grid.len() {
                queue.push_back(((row + 1, col), dist + 1));
            }

            // Left
            if col != 0 {
                queue.push_back(((row, col - 1), dist + 1));
            }

            // Top
            if row != 0 {
                queue.push_back(((row - 1, col), dist + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut grid = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        Solution::islandsAndTreasure(&mut grid);
        assert_eq!(
            grid,
            vec![
                vec![3, -1, 0, 1],
                vec![2, 2, 1, -1],
                vec![1, -1, 2, -1],
                vec![0, -1, 3, 4]
            ]
        );

        grid = vec![vec![0, -1], vec![2147483647, 2147483647]];
        Solution::islandsAndTreasure(&mut grid);
        assert_eq!(grid, vec![vec![0, -1], vec![1, 2]]);
    }
}
