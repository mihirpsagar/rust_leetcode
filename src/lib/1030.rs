use std::collections::VecDeque;

// Time taken: 13:34, 13:39, 13:46, 14:05 -> Acc
struct Solution {}

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut visited = vec![];

        for _ in 0..rows {
            visited.push(vec![false; cols as usize]);
        }

        let mut result = Vec::new();
        queue.push_back((r_center, c_center));
        visited[r_center as usize][c_center as usize] = true;

        while let Some((r1, c1)) = queue.pop_front() {
            result.push(vec![r1, c1]);

            if r1 != 0 {
                if !visited[r1 as usize - 1][c1 as usize] {
                    queue.push_back((r1 - 1, c1));
                    visited[r1 as usize - 1][c1 as usize] = true;
                }
            }
            if c1 != cols - 1 {
                if !visited[r1 as usize][c1 as usize + 1] {
                    queue.push_back((r1, c1 + 1));
                    visited[r1 as usize][c1 as usize + 1] = true;
                }
            }
            if r1 != rows - 1 {
                if !visited[r1 as usize + 1][c1 as usize] {
                    queue.push_back((r1 + 1, c1));
                    visited[r1 as usize + 1][c1 as usize] = true;
                }
            }
            if c1 != 0 {
                if !visited[r1 as usize][c1 as usize - 1] {
                    queue.push_back((r1, c1 - 1));
                    visited[r1 as usize][c1 as usize - 1] = true;
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::all_cells_dist_order(1, 2, 0, 0), [[0, 0], [0, 1]]);
        assert_eq!(
            Solution::all_cells_dist_order(2, 2, 0, 1),
            [[0, 1], [0, 0], [1, 1], [1, 0]]
        );
        assert_eq!(
            Solution::all_cells_dist_order(2, 3, 1, 2),
            [[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]]
        );
    }
}
