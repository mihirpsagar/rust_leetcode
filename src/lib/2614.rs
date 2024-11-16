use std::collections::{HashMap, HashSet};

// Time taken: 20:47, 20:52 -> TLE, 20:57 -> Acc, 22:14, 22:20 -> Optimized
struct Solution {}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut idx1 = 0;
        let mut idx2 = nums[0].len() - 1;
        let mut arr = Vec::new();

        while idx1 < nums.len() {
            Self::binary_insert_unique(&mut arr, nums[idx1][idx1]);
            Self::binary_insert_unique(&mut arr, nums[idx1][idx2]);
            idx1 += 1;
            idx2 = idx2.saturating_sub(1);
        }

        let mut idx = arr.len() - 1;
        loop {
            if Self::is_prime(arr[idx]) {
                return arr[idx];
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return 0;
    }

    pub fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }

        let mut val = 3;
        let threshold = (num as f64).sqrt().round() as i32 + 1;

        while val < threshold {
            if num % val == 0 {
                return false;
            }
            val += 2;
        }

        return true;
    }

    pub fn binary_insert_unique(arr: &mut Vec<i32>, target: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] == target {
                return;
            } else if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        arr.insert(left, target);
    }

    // pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    //     let mut idx1 = 0;
    //     let mut idx2 = nums[0].len() - 1;
    //     let mut result = 0;
    //     let mut map = HashMap::new();

    //     while idx1 < nums.len() {
    //         if Self::is_prime(nums[idx1][idx1], &mut map) && nums[idx1][idx1] > result {
    //             result = nums[idx1][idx1];
    //         }
    //         if Self::is_prime(nums[idx1][idx2], &mut map) && nums[idx1][idx2] > result {
    //             result = nums[idx1][idx2];
    //         }
    //         idx1 += 1;
    //         idx2 -= 1;
    //     }

    //     return result;
    // }

    // pub fn is_prime(num: i32, map: &mut HashMap<i32, bool>) -> bool {
    //     if num == 1 {
    //         return false;
    //     }
    //     if let Some(val) = map.get(&num) {
    //         return *val;
    //     } else {
    //         let mut val = 2;
    //         let threshold = (num / 2) + 1;
    //         while val < threshold {
    //             if num % val == 0 {
    //                 map.insert(num, false);
    //                 return false;
    //             }
    //             val += 1;
    //         }
    //         map.insert(num, true);
    //         return true;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]),
            11
        );
        assert_eq!(
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]),
            17
        );
    }
}
