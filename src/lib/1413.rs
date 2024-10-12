// Time taken: 11:11, 11:15 -> Acc
struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut sum = 0;
        let mut idx = 0;

        while idx < nums.len() {
            sum += nums[idx];
            if sum < 1 {
                let min = 1 - sum;
                if min > result {
                    result = min;
                }
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
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
