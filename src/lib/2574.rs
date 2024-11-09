// Time taken: 07:55, 07:57 -> Acc
struct Solution {}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum: i32 = 0;
        let mut right_sum: i32 = nums.iter().sum();
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            right_sum -= nums[idx];
            result.push(left_sum.abs_diff(right_sum) as i32);
            left_sum += nums[idx];
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
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            [15, 1, 11, 22]
        );
        assert_eq!(Solution::left_right_difference(vec![1]), [0]);
    }
}
