use std::collections::HashMap;

// Time taken: 16:58, 17:02 -> Acc
struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut result = vec![0, 0];
        let mut map = HashMap::new();

        for row in grid {
            for val in row {
                *map.entry(val).or_insert(0) += 1;
            }
        }

        let limit = (n * n) as i32;
        for k in 0..=limit {
            if let Some(val) = map.get(&k) {
                if *val == 2 {
                    result[0] = k;
                }
            } else {
                result[1] = k;
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
            Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            [2, 4]
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![9, 1, 7],
                vec![8, 9, 2],
                vec![3, 4, 6]
            ]),
            [9, 5]
        );
    }
}
