// Time taken: 14:32, 14:33 -> Acc

struct Solution {}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut idx = 0;
        let mut result = 0;
        while idx < len {
            if len % (idx + 1) == 0 {
                result = result + (nums[idx] * nums[idx]);
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
        assert_eq!(Solution::sum_of_squares(vec![1, 2, 3, 4]), 21);
        assert_eq!(Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
