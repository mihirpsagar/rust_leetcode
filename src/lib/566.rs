// Time taken: 19:42, 19:59 -> Acc
struct Solution {}

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if (r * c) as usize != (mat.len() * mat[0].len()) {
            return mat;
        }

        let mut result = Vec::new();
        let mut ptr = 0;
        let mat_col = mat[0].len();

        for _ in 0..r {
            let mut row = Vec::new();
            for _ in 0..c {
                row.push(mat[ptr / mat_col][ptr % mat_col]);
                ptr += 1;
            }
            result.push(row);
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
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }
}
