// Time taken: 22:58, 23:16 -> Wrong, 23:19 -> Acc
struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut count_key = 0;
        let mut left = 0 - k;
        let mut right = std::cmp::min(k, nums.len() as i32 - 1);
        for idx in 0..=right {
            if nums[idx as usize] == key {
                count_key += 1;
            }
        }

        let mut idx = 0;
        while idx < nums.len() {
            if count_key > 0 {
                result.push(idx as i32);
            }
            if left >= 0 {
                if nums[left as usize] == key {
                    count_key -= 1;
                }
            }
            left += 1;
            right += 1;
            if (right as usize) < nums.len() {
                if nums[right as usize] == key {
                    count_key += 1;
                }
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
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            [1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::find_k_distant_indices(vec![1], 1, 1), [0]);
    }
}
