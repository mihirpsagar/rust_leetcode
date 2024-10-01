// Time taken: 23:03, 23:06 -> Acc
struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut y = 0;
        while y < matrix[0].len() {
            let mut x = 0;
            let mut vec = Vec::new();
            while x < matrix.len() {
                vec.push(matrix[x][y]);
                x += 1;
            }
            result.push(vec);
            y += 1;
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
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        );
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![[1, 4], [2, 5], [3, 6]]
        );
    }
}
