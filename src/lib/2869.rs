use std::collections::HashSet;

// Time taken: 10:51, 10:54 -> Acc
struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut result = 0;
        let mut idx = nums.len() - 1;

        loop {
            if nums[idx] <= k {
                set.insert(nums[idx]);
            }
            result += 1;

            if set.len() == k as usize {
                break;
            }

            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 5), 5);
        assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
