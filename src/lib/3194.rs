// Time taken: 11:49, 11:52 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        let mut result = f64::MAX;
        let mut left = 0;
        let mut right = nums.len() - 1;

        nums.sort();

        while left < right {
            let val = (nums[left] + nums[right]) as f64 / 2.0;
            if val < result {
                result = val;
            }
            left += 1;
            right -= 1;
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
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
            5.5
        );
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
