// Time taken: 10:33, 10:37 -> Acc
struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut longest_increasing = 1;
        let mut left = 0;
        let mut right = 1;

        while right < nums.len() {
            if nums[right] > nums[right - 1] {
                longest_increasing = std::cmp::max(longest_increasing, right - left + 1);
            } else {
                left = right;
            }
            right += 1;
        }

        let mut longest_decreasing = 1;
        left = 0;
        right = 1;
        while right < nums.len() {
            if nums[right] < nums[right - 1] {
                longest_decreasing = std::cmp::max(longest_decreasing, right - left + 1);
            } else {
                left = right;
            }
            right += 1;
        }

        return std::cmp::max(longest_increasing, longest_decreasing) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
