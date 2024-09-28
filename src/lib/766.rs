// Time taken: 00:37, 00:41
struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut prev_row = &matrix[0];

        for row in matrix.iter().skip(1) {
            let mut p1 = 0;
            let mut p2 = 1;
            while p2 < row.len() {
                if prev_row[p1] != row[p2] {
                    return false;
                }
                p1 += 1;
                p2 += 1;
            }
            prev_row = row;
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
            Solution::is_toeplitz_matrix(vec![
                vec![1, 2, 3, 4],
                vec![5, 1, 2, 3],
                vec![9, 5, 1, 2]
            ]),
            true
        );
        assert_eq!(
            Solution::is_toeplitz_matrix(vec![vec![1, 1], vec![2, 2]]),
            false
        );
    }
}
