// Time taken: 23:38, 23:41 -> Acc
struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut idx = 2;
        let sum = nums[0] + nums[1];

        while idx < nums.len() - 1 {
            if (nums[idx] + nums[idx + 1]) != sum {
                break;
            }
            result += 1;
            idx += 2;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(
            Solution::max_operations(vec![1, 5, 3, 3, 4, 1, 3, 2, 2, 3]),
            2
        );
        assert_eq!(Solution::max_operations(vec![5, 3]), 1);
    }
}
