// Time taken: 01:54, 02:00 -> Acc
struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut idx1 = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = Vec::new();

        'outer: while idx1 < m {
            let mut smallest_idx = 0;
            let mut idx2 = 1;
            while idx2 < n {
                if matrix[idx1][idx2] < matrix[idx1][smallest_idx] {
                    smallest_idx = idx2;
                }
                idx2 += 1;
            }

            idx2 = 0;
            while idx2 < m {
                if matrix[idx2][smallest_idx] > matrix[idx1][smallest_idx] {
                    idx1 += 1;
                    continue 'outer;
                }
                idx2 += 1;
            }

            result.push(matrix[idx1][smallest_idx]);
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
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            [15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            [12]
        );
        assert_eq!(Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]), [7]);
    }
}
