use std::collections::HashSet;

// Time taken: 16:24, 16:31 -> Wrong, 16:36 -> Acc, 16:46 -> Optimized
struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let mut set = HashSet::new();
        let mut left = 0;
        let mut right = 1;
        let k = k as usize;

        while right <= nums.len() {
            while right < nums.len() && nums[right] > nums[right - 1] && right - left + 1 <= k {
                right += 1;
            }

            if (right - left) == k {
                if set.contains(&left.saturating_sub(1)) {
                    return true;
                }
                set.insert(right - 1);
            }

            left += 1;
            right = left + 1;
        }

        return false;
    }

    // pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    //     let mut idx = 0;
    //     let k = k as usize;

    //     while idx < nums.len() {
    //         if Self::is_valid(&nums, idx, k) {
    //             if Self::is_valid(&nums, idx + k, k) {
    //                 return true;
    //             }
    //         }
    //         idx += 1;
    //     }

    //     return false;
    // }

    // pub fn is_valid(arr: &Vec<i32>, start: usize, k: usize) -> bool {
    //     let end = start + k - 1;
    //     if end >= arr.len() {
    //         return false;
    //     }
    //     let mut idx = start + 1;
    //     while idx <= end {
    //         if arr[idx] <= arr[idx - 1] {
    //             return false;
    //         }
    //         idx += 1;
    //     }

    //     return true;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::has_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5),
            false
        );
        assert_eq!(Solution::has_increasing_subarrays(vec![-15, 19], 1), true);
    }
}
