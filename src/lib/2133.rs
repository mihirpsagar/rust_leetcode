// Time taken: 21:20, 21:26 -> Acc
struct Solution {}

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let len = matrix.len();
        let mut row_count = Vec::new();
        let mut col_count = Vec::new();
        for _ in 0..len {
            row_count.push(vec![0; len]);
            col_count.push(vec![0; len]);
        }

        let mut idx1 = 0;
        while idx1 < len {
            let mut idx2 = 0;
            while idx2 < len {
                let val = matrix[idx1][idx2] as usize - 1;

                row_count[idx1][val] += 1;
                if row_count[idx1][val] == 2 {
                    return false;
                }

                col_count[idx2][val] += 1;
                if col_count[idx2][val] == 2 {
                    return false;
                }

                idx2 += 1;
            }

            idx1 += 1;
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
            Solution::check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]]),
            true
        );
        assert_eq!(
            Solution::check_valid(vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]]),
            false
        );
    }
}
