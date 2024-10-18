use std::collections::HashMap;

// Time taken: 00:09, 00:10 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut result = 0;

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for (key, val) in map {
            if val == 1 {
                result += key;
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
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
