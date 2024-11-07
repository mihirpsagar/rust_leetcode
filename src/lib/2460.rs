// Time taken: 03:32, 03:36 -> Acc
struct Solution {}

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut idx = 0;

        while idx < nums.len() - 1 {
            if nums[idx] == nums[idx + 1] {
                nums[idx] *= 2;
                nums[idx + 1] = 0;
            }
            idx += 1;
        }

        let mut left = 0;
        idx = 0;
        while idx < nums.len() {
            if nums[idx] != 0 {
                let tmp = nums[left];
                nums[left] = nums[idx];
                nums[idx] = tmp;
                left += 1;
            }
            idx += 1;
        }

        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            [1, 4, 2, 0, 0, 0]
        );
        assert_eq!(Solution::apply_operations(vec![0, 1]), [1, 0]);
    }
}
