// Time taken: 20:41, 20:44 -> Acc
struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx = 1;

        while idx < nums.len() {
            if nums[idx] <= nums[idx - 1] {
                result = result + (nums[idx - 1] - nums[idx] + 1);
                nums[idx] = nums[idx - 1] + 1;
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }
}
