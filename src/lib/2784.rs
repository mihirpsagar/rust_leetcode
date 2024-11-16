// Time taken: 21:00, 21:07 -> Acc

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let mut max = nums[0];
        let len = nums.len();
        for num in nums {
            if num > max {
                max = num;
            }
            *map.entry(num).or_insert(0) += 1;
        }

        if len != max as usize + 1 {
            return false;
        }

        let mut key = 1;
        while key < max {
            if let Some(val) = map.get(&key) {
                if *val != 1 {
                    return false;
                }
            } else {
                return false;
            }
            key += 1;
        }

        if let Some(val) = map.get(&key) {
            if *val != 2 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_good(vec![2, 1, 3]), false);
        assert_eq!(Solution::is_good(vec![1, 3, 3, 2]), true);
        assert_eq!(Solution::is_good(vec![1, 1]), true);
        assert_eq!(Solution::is_good(vec![3, 4, 4, 1, 2, 1]), false);
    }
}
