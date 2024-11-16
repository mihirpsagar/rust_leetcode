// Time taken: 09:42, 09:44 -> Acc
struct Solution {}

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0, 0];

        for (idx, row) in mat.iter().enumerate() {
            let mut count = 0;
            for num in row {
                if *num == 1 {
                    count += 1;
                }
            }
            if count > result[1] {
                result[0] = idx as i32;
                result[1] = count;
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
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            [0, 1]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            [1, 2]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            [1, 2]
        );
    }
}
