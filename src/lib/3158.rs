use std::collections::HashMap;

// Time taken: 11:22, 11:24 -> Acc
struct Solution {}

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result = 0;
        for (key, val) in map {
            if val == 2 {
                result ^= key;
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
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
