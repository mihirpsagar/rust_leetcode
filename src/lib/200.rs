// Time taken: 12:02, 12:11 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;
        while idx1 < grid.len() {
            let mut idx2 = 0;
            while idx2 < grid[0].len() {
                if Self::check_island(idx1, idx2, &mut grid) {
                    result += 1;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return result;
    }

    fn check_island(row: usize, col: usize, mut grid: &mut Vec<Vec<char>>) -> bool {
        // Base case
        // if row >= grid.len() || col >= grid[0].len() {
        //     return false;
        // }
        if grid[row][col] == '0' {
            return false;
        }

        grid[row][col] = '0';

        // Right
        if col + 1 < grid[0].len() && grid[row][col + 1] == '1' {
            Self::check_island(row, col + 1, &mut grid);
        }

        // Bottom
        if row + 1 < grid.len() && grid[row + 1][col] == '1' {
            Self::check_island(row + 1, col, &mut grid);
        }

        // Left
        if col != 0 && grid[row][col - 1] == '1' {
            Self::check_island(row, col - 1, &mut grid);
        }

        // Top
        if row != 0 && grid[row - 1][col] == '1' {
            Self::check_island(row - 1, col, &mut grid);
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
