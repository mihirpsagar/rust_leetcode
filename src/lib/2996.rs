// Time taken: 17:22, 17:28 -> Acc
struct Solution {}

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut idx = 1;

        while idx < nums.len() {
            if nums[idx - 1] + 1 == nums[idx] {
                sum += nums[idx];
            } else {
                break;
            }
            idx += 1;
        }

        while nums.contains(&sum) {
            sum += 1;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }
}
