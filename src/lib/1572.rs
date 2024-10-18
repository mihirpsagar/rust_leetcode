// Time taken: 19:11, 19:18 -> Acc
struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        if mat[0].len() == 1 {
            return mat[0][0];
        }

        let mut sum = 0;
        let mut i = 0;
        let mut j = mat[0].len() - 1;

        while i < mat.len() {
            sum += mat[i][i];
            sum += mat[i][j];
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }

        if mat.len() % 2 != 0 {
            let tmp = mat.len() / 2;
            sum -= mat[tmp][tmp];
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
        assert_eq!(
            Solution::diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8
        );
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }
}
