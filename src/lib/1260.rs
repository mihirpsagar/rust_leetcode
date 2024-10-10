// Time taken: 22:23, 22:39 -> Acc
struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let m = grid.len();
        let n = grid[0].len();
        let size = m * n;

        for _ in 0..m {
            result.push(vec![0; n]);
        }

        let mut idx1 = 0;
        while idx1 < m {
            let mut idx2 = 0;
            while idx2 < n {
                let mut pos = (n * idx1) + idx2;
                pos = (pos + k as usize) % size;
                let new_idx1 = pos / n;
                let new_idx2 = pos % n;
                result[new_idx1][new_idx2] = grid[idx1][idx2];
                idx2 += 1;
            }
            idx1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            [[9, 1, 2], [3, 4, 5], [6, 7, 8]]
        );
        assert_eq!(
            Solution::shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13]
                ],
                4
            ),
            [[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
        );
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        );
    }
}
