use std::collections::HashSet;

// Time taken: 23:20, 23:22 -> Acc
struct Solution {}

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        let mut idx = 1;

        while idx < nums.len() {
            if set.contains(&(nums[idx] + nums[idx - 1])) {
                return true;
            }
            set.insert(nums[idx] + nums[idx - 1]);
            idx += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_subarrays(vec![4, 2, 4]), true);
        assert_eq!(Solution::find_subarrays(vec![1, 2, 3, 4, 5]), false);
        assert_eq!(Solution::find_subarrays(vec![0, 0, 0]), true);
    }
}
