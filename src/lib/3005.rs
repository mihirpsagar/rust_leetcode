use std::collections::HashMap;

// Time taken: 17:43, 17:45 -> Acc
struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut result = (0, 0); //(freq, sum)
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for val in map.values() {
            if *val > result.0 {
                result = (*val, *val);
            } else if *val == result.0 {
                result.1 += *val;
            }
        }

        return result.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
