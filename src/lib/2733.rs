// Time taken: 11:28, 11:32 -> Acc
struct Solution {}

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }

        let mut idx = 1;
        let mut range = (nums[0], nums[0]);

        while idx < nums.len() {
            if nums[idx] < range.0 {
                range.0 = nums[idx];
            } else if nums[idx] > range.1 {
                range.1 = nums[idx];
            } else {
                return nums[idx];
            }
            idx += 1;
        }

        idx = 0;
        while idx < nums.len() {
            if nums[idx] != range.0 && nums[idx] != range.1 {
                return nums[idx];
            }
            idx += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_non_min_or_max(vec![3, 2, 1, 4]), 3);
        assert_eq!(Solution::find_non_min_or_max(vec![1, 2]), -1);
        assert_eq!(Solution::find_non_min_or_max(vec![2, 1, 3]), 2);
    }
}
