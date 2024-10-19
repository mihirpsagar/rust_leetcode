// Time taken: 23:55, 23:59 -> Acc
struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx = 0;
        let mut prev = -1;
        let mut sum = 0;

        while idx < nums.len() {
            if nums[idx] > prev {
                sum += nums[idx];
            } else {
                sum = nums[idx];
            }

            if sum > result {
                result = sum;
            }

            prev = nums[idx];
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
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
            33
        );
    }
}
