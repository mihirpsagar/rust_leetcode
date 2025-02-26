// Time taken: 14:34, 14:52, 14:55 -> Acc, 15:22 -> Optimized
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    // Time complexity: O(m * n)
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for _ in 0..heights.len() {
            result.push(vec![(false, false); heights[0].len()]);
        }

        // BFS for pacific
        let mut visited = HashSet::new();
        for col in 0..heights[0].len() {
            Self::bfs(0, col, &heights, &mut visited, &mut result, true);
        }
        for row in 0..heights.len() {
            Self::bfs(row, 0, &heights, &mut visited, &mut result, true);
        }

        // BFS for atlantic
        let mut visited = HashSet::new();
        for row in 0..heights.len() {
            Self::bfs(
                row,
                heights[0].len() - 1,
                &heights,
                &mut visited,
                &mut result,
                false,
            );
        }
        for col in 0..heights[0].len() {
            Self::bfs(
                heights.len() - 1,
                col,
                &heights,
                &mut visited,
                &mut result,
                false,
            );
        }

        let mut result_arr = Vec::new();

        let mut idx1 = 0;
        while idx1 < heights.len() {
            let mut idx2 = 0;
            while idx2 < heights[0].len() {
                if result[idx1][idx2].0 && result[idx1][idx2].1 {
                    result_arr.push(vec![idx1 as i32, idx2 as i32]);
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return result_arr;
    }

    fn bfs(
        row: usize,
        col: usize,
        heights: &Vec<Vec<i32>>,
        mut visited: &mut HashSet<(usize, usize)>,
        mut result: &mut Vec<Vec<(bool, bool)>>,
        is_pacific: bool,
    ) {
        // Base case
        if visited.contains(&(row, col)) {
            return;
        }

        if is_pacific {
            result[row][col].0 = true;
        } else {
            result[row][col].1 = true;
        }

        visited.insert((row, col));

        // Right
        if col + 1 < heights[0].len() && heights[row][col + 1] >= heights[row][col] {
            Self::bfs(
                row,
                col + 1,
                &heights,
                &mut visited,
                &mut result,
                is_pacific,
            );
        }

        // Bottom
        if row + 1 < heights.len() && heights[row + 1][col] >= heights[row][col] {
            Self::bfs(
                row + 1,
                col,
                &heights,
                &mut visited,
                &mut result,
                is_pacific,
            );
        }

        // Left
        if col != 0 && heights[row][col - 1] >= heights[row][col] {
            Self::bfs(
                row,
                col - 1,
                &heights,
                &mut visited,
                &mut result,
                is_pacific,
            );
        }

        // Top
        if row != 0 && heights[row - 1][col] >= heights[row][col] {
            Self::bfs(
                row - 1,
                col,
                &heights,
                &mut visited,
                &mut result,
                is_pacific,
            );
        }
    }

    // Time complexity: O((m*n)^2)
    // pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     let mut result = Vec::new();
    //     let mut idx1 = 0;
    //     while idx1 < heights.len() {
    //         let mut idx2 = 0;
    //         while idx2 < heights[0].len() {
    //             let val = Self::dfs(idx1, idx2, &heights, &mut HashSet::new());
    //             if val.0 && val.1 {
    //                 result.push(vec![idx1 as i32, idx2 as i32]);
    //             }
    //             idx2 += 1;
    //         }
    //         idx1 += 1;
    //     }

    //     return result;
    // }

    // fn dfs(
    //     row: usize,
    //     col: usize,
    //     heights: &Vec<Vec<i32>>,
    //     mut visited: &mut HashSet<(usize, usize)>,
    // ) -> (bool, bool) {
    //     // Base case
    //     if row == 0 && col + 1 == heights[0].len() {
    //         return (true, true);
    //     }
    //     if col == 0 && row + 1 == heights.len() {
    //         return (true, true);
    //     }

    //     let mut result1 = false;
    //     let mut result2 = false;

    //     if row == 0 || col == 0 {
    //         result1 = true;
    //     }
    //     if row + 1 == heights.len() || col + 1 == heights[0].len() {
    //         result2 = true;
    //     }

    //     visited.insert((row, col));

    //     // Top
    //     if (!result1 || !result2) && row != 0 && heights[row - 1][col] <= heights[row][col] {
    //         if !visited.contains(&(row - 1, col)) {
    //             let val = Self::dfs(row - 1, col, &heights, &mut visited);
    //             result1 = result1 | val.0;
    //             result2 = result2 | val.1;
    //         }
    //     }

    //     // Left
    //     if (!result1 || !result2) && col != 0 && heights[row][col - 1] <= heights[row][col] {
    //         if !visited.contains(&(row, col - 1)) {
    //             let val = Self::dfs(row, col - 1, &heights, &mut visited);
    //             result1 = result1 | val.0;
    //             result2 = result2 | val.1;
    //         }
    //     }

    //     // Right
    //     if (!result1 || !result2)
    //         && col + 1 < heights[0].len()
    //         && heights[row][col + 1] <= heights[row][col]
    //     {
    //         if !visited.contains(&(row, col + 1)) {
    //             let val = Self::dfs(row, col + 1, &heights, &mut visited);
    //             result1 = result1 | val.0;
    //             result2 = result2 | val.1;
    //         }
    //     }

    //     // Bottom
    //     if (!result1 || !result2)
    //         && row + 1 < heights.len()
    //         && heights[row + 1][col] <= heights[row][col]
    //     {
    //         if !visited.contains(&(row + 1, col)) {
    //             let val = Self::dfs(row + 1, col, &heights, &mut visited);
    //             result1 = result1 | val.0;
    //             result2 = result2 | val.1;
    //         }
    //     }

    //     return (result1, result2);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        );
    }
}
