// Time taken: 13:57, 14:19 -> Acc, 13:55 -> Optimized

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            let tmp = nums1;
            nums1 = nums2;
            nums2 = tmp;
        }

        let mut left = 0;
        let mut right = nums1.len();
        let total = nums1.len() + nums2.len();
        let half_len = (total / 2) + (total % 2);

        loop {
            let mut val1_left = i32::MIN;
            let mut val1_right = i32::MAX;
            let mut val2_left = i32::MIN;
            let mut val2_right = i32::MAX;
            let mut left_arr_count = 0;

            // println!("Left, Right {} {}", left, right);
            if left < right {
                let mid = left + (right - left) / 2;
                // if mid == 0 {
                //     val1_right = nums1[mid];
                // } else {
                val1_left = nums1[mid];
                if mid + 1 < nums1.len() {
                    val1_right = nums1[mid + 1];
                }
                // }
                left_arr_count = mid + 1;
            } else {
                if left < nums1.len() {
                    val1_right = nums1[left];
                }
            }

            // println!("Half, Left {} {}", half_len, left_arr_count);
            if half_len == left_arr_count {
                val2_right = nums2[0];
            } else {
                let right_mid = half_len - left_arr_count - 1;
                if right_mid < nums2.len() {
                    val2_left = nums2[right_mid];
                }
                if right_mid + 1 < nums2.len() {
                    val2_right = nums2[right_mid + 1];
                }
            }

            // println!("{} {} {} {}", val1_left, val1_right, val2_left, val2_right);

            if val1_left <= val2_right && val2_left <= val1_right {
                if total % 2 != 0 {
                    return std::cmp::max(val1_left, val2_left) as f64;
                } else {
                    return (std::cmp::max(val1_left, val2_left)
                        + std::cmp::min(val1_right, val2_right)) as f64
                        / 2 as f64;
                }
            }

            if left < right {
                if val1_left > val2_right {
                    if left_arr_count != 0 {
                        right = left_arr_count - 1;
                    } else {
                        right = 0;
                    }
                } else {
                    left = left_arr_count;
                }
            }
        }
    }

    // pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let mut idx1 = 0;
    //     let mut idx2 = 0;
    //     let total_len = nums1.len() + nums2.len();
    //     let mut count = total_len / 2;
    //     if total_len % 2 != 0 {
    //         count += 1;
    //     }

    //     for _ in (1..count) {
    //         let val1 = if idx1 < nums1.len() {
    //             nums1[idx1]
    //         } else {
    //             i32::MAX
    //         };
    //         let val2 = if idx2 < nums2.len() {
    //             nums2[idx2]
    //         } else {
    //             i32::MAX
    //         };

    //         if val1 <= val2 {
    //             idx1 += 1;
    //         } else {
    //             idx2 += 1;
    //         }
    //     }

    //     if total_len % 2 != 0 {
    //         let val1 = if idx1 < nums1.len() {
    //             nums1[idx1]
    //         } else {
    //             i32::MAX
    //         };
    //         let val2 = if idx2 < nums2.len() {
    //             nums2[idx2]
    //         } else {
    //             i32::MAX
    //         };

    //         return std::cmp::min(val1, val2) as f64;
    //     } else {
    //         let mut arr = Vec::new();
    //         if idx1 < nums1.len() {
    //             arr.push(nums1[idx1]);
    //         }
    //         if idx1 + 1 < nums1.len() {
    //             arr.push(nums1[idx1 + 1]);
    //         }
    //         if idx2 < nums2.len() {
    //             arr.push(nums2[idx2]);
    //         }
    //         if idx2 + 1 < nums2.len() {
    //             arr.push(nums2[idx2 + 1]);
    //         }
    //         arr.sort();

    //         return (arr[0] + arr[1]) as f64 / 2 as f64;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![100001], vec![100000]),
            100000.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3], vec![1, 2, 4]),
            2.5
        );
    }
}
