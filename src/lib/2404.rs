use std::collections::HashMap;

// Time taken: 23:29, 23:33 -> Acc
struct Solution {}

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut result: Option<(i32, i32)> = None;

        for num in nums {
            if num % 2 == 0 {
                *map.entry(num).or_insert(0) += 1;
            }
        }

        for (key, val) in map {
            if let Some(prev) = result {
                if val > prev.1 {
                    result = Some((key, val));
                } else if val == prev.1 && key < prev.0 {
                    result = Some((key, val));
                }
            } else {
                result = Some((key, val));
            }
        }

        if let Some(val) = result {
            return val.0;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
        assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
        assert_eq!(
            Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]),
            -1
        );
    }
}
