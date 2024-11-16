// Time taken: 11:14, 11:19 -> Acc
struct Solution {}

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = nums[0];
        let mut idx = 1;
        while idx < nums.len() {
            if nums[idx] > max {
                max = nums[idx];
            }
            idx += 1;
        }

        let sum1 = ((max + k - 1) * (max + k)) / 2;
        let sum2 = (max * (max - 1)) / 2;
        return sum1 - sum2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(Solution::maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
