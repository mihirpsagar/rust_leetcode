// Time taken: 12:15, 12:21 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;
        while idx1 < grid.len() {
            let mut idx2 = 0;
            while idx2 < grid[0].len() {
                if grid[idx1][idx2] == 1 {
                    result = std::cmp::max(result, Self::dfs(idx1, idx2, &mut grid));
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return result;
    }

    fn dfs(row: usize, col: usize, mut grid: &mut Vec<Vec<i32>>) -> i32 {
        // Base case
        if grid[row][col] == 0 {
            return 0;
        }

        grid[row][col] = 0;
        let mut count = 1;

        // Right
        if col + 1 < grid[0].len() && grid[row][col + 1] == 1 {
            count += Self::dfs(row, col + 1, &mut grid);
        }

        // Bottom
        if row + 1 < grid.len() && grid[row + 1][col] == 1 {
            count += Self::dfs(row + 1, col, &mut grid);
        }

        // Left
        if col != 0 && grid[row][col - 1] == 1 {
            count += Self::dfs(row, col - 1, &mut grid);
        }

        // Top
        if row != 0 && grid[row - 1][col] == 1 {
            count += Self::dfs(row - 1, col, &mut grid);
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );
    }
}
