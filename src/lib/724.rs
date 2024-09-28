// Time taken: 20:39, 20:44, 20:47 -> Acc

struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut idx = 0;
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();

        while idx < nums.len() {
            if idx > 0 {
                left_sum += nums[idx - 1];
            }
            right_sum -= nums[idx];
            if left_sum == right_sum {
                return idx as i32;
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
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
