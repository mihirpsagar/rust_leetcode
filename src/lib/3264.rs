// Time taken: 13:21, 13:23 -> Acc
struct Solution {}

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            let mut min_idx = 0;
            let mut idx = 1;
            while idx < nums.len() {
                if nums[idx] < nums[min_idx] {
                    min_idx = idx;
                }
                idx += 1;
            }
            nums[min_idx] *= multiplier;
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
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            [8, 4, 6, 5, 6]
        );
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), [16, 8]);
    }
}
