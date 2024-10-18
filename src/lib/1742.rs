use std::collections::HashMap;

// Time taken: 00:00, 00:07 -> Acc
struct Solution {}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut map = HashMap::new();

        for num in low_limit..=high_limit {
            let mut sum = 0;
            let mut val = num;
            while val > 0 {
                sum = sum + (val % 10);
                val /= 10;
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        let mut result = 0;

        for &val in map.values() {
            if val > result {
                result = val;
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
        assert_eq!(Solution::count_balls(1, 10), 2);
        assert_eq!(Solution::count_balls(5, 15), 2);
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
