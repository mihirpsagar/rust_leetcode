// Time taken: 15:02, 15:11 -> Acc
struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut nums = nums;
        let mut idx = 0;

        while idx < nums.len() {
            let p_idx = (nums[idx].abs() - 1) as usize;
            if nums[p_idx] > 0 {
                nums[p_idx] *= -1;
            }
            idx += 1;
        }

        idx = 0;

        while idx < nums.len() {
            if nums[idx] > 0 {
                result.push((idx + 1) as i32);
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
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
