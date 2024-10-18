// Time taken: 20:51, 21:01 -> Wrong, 21:05 -> Wrong, 21:11 -> Wrong, 11:17 -> Wrong, 11:20 -> Acc
struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut row_sum = vec![(0, 0); m];
        let mut col_sum = vec![(0, 0); n];
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < m {
            let mut idx2 = 0;
            while idx2 < n {
                if mat[idx1][idx2] == 1 {
                    row_sum[idx1].0 += 1;
                    row_sum[idx1].1 = idx2;
                    col_sum[idx2].0 += 1;
                    col_sum[idx2].1 = idx1;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        idx1 = 0;
        while idx1 < m {
            if row_sum[idx1].0 != 1 {
                idx1 += 1;
                continue;
            }
            if col_sum[row_sum[idx1].1].0 == 1 {
                result += 1;
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
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 1],
                vec![1, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1]
            ]),
            2
        );
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0]
            ]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0]
            ]),
            2
        );
    }
}
