// Time taken: 23:30, 23:36 -> Acc
struct Solution {}

impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut col_max = vec![0; n];

        let mut idx1 = 0;
        while idx1 < n {
            let mut idx2 = 0;
            let mut max = -1;
            while idx2 < m {
                max = std::cmp::max(matrix[idx2][idx1], max);
                idx2 += 1;
            }
            col_max[idx1] = max;
            idx1 += 1;
        }

        idx1 = 0;
        while idx1 < m {
            let mut idx2 = 0;
            while idx2 < n {
                if matrix[idx1][idx2] == -1 {
                    matrix[idx1][idx2] = col_max[idx2];
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
            [[1, 2, 9], [4, 8, 6], [7, 8, 9]]
        );
        assert_eq!(
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
            [[3, 2], [5, 2]]
        );
    }
}
