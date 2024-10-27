// Time taken: 12:33, 12:36 -> Acc
struct Solution {}

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut right_sum = nums.iter().sum::<i32>();

        for (idx, &num) in nums.iter().enumerate() {
            if idx != 0 {
                left_sum += nums[idx - 1];
            }
            right_sum -= num;

            if left_sum == right_sum {
                return idx as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
        assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
        assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
    }
}
