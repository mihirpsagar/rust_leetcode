// Time taken: 17:47, 17:50 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut min = (i32::MAX, i32::MAX);
        let mut idx = 1;

        while idx < nums.len() {
            if nums[idx] <= min.0 {
                min.1 = min.0;
                min.0 = nums[idx];
            } else if nums[idx] < min.1 {
                min.1 = nums[idx];
            }

            idx += 1;
        }

        return nums[0] + min.0 + min.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
        assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
        assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
    }
}
