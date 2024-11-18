// Time taken: 15:14, 15:25 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut found = false;
        let mut min_left = nums[0];
        let mut idx = 1;

        while idx < nums.len() - 1 {
            if nums[idx] <= min_left {
                min_left = nums[idx];
                idx += 1;
                continue;
            }

            if let Some(min_right) = Self::get_min_right(&nums, idx) {
                found = true;
                result = std::cmp::min(result, min_left + nums[idx] + min_right);
            }

            idx += 1;
        }

        if found {
            return result;
        } else {
            return -1;
        }
    }

    pub fn get_min_right(arr: &Vec<i32>, peak_idx: usize) -> Option<i32> {
        let mut idx = peak_idx + 1;
        let mut result = i32::MAX;
        let mut found = false;
        while idx < arr.len() {
            if arr[idx] < arr[peak_idx] && arr[idx] < result {
                found = true;
                result = arr[idx];
            }
            idx += 1;
        }

        if found {
            return Some(result);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
