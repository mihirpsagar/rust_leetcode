// Time taken: 00:17, 00:24 -> Wrong, 00:29 -> Acc
struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut max_row = m;
        let mut max_col = n;

        for val in ops {
            max_row = std::cmp::min(max_row, val[0]);
            max_col = std::cmp::min(max_col, val[1]);
        }

        return max_row * max_col;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_count(
                18,
                3,
                vec![
                    vec![16, 1],
                    vec![14, 3],
                    vec![14, 2],
                    vec![4, 1],
                    vec![10, 1],
                    vec![11, 1],
                    vec![8, 3],
                    vec![16, 2],
                    vec![13, 1],
                    vec![8, 3],
                    vec![2, 2],
                    vec![9, 1],
                    vec![3, 1],
                    vec![2, 2],
                    vec![6, 3]
                ]
            ),
            2
        );
    }
}
