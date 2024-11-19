// Time taken: 11:20, 11:21 -> Acc
struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut prev = nums[0] % 2;
        let mut idx = 1;

        while idx < nums.len() {
            if (nums[idx] % 2) == prev {
                return false;
            }
            prev = nums[idx] % 2;
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_array_special(vec![1]), true);
        assert_eq!(Solution::is_array_special(vec![2, 1, 4]), true);
        assert_eq!(Solution::is_array_special(vec![4, 3, 1, 6]), false);
    }
}
