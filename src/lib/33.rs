// Time taken: 13:04, 13:28 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                // Left side
                if target > nums[mid] {
                    left = mid + 1;
                } else {
                    if target >= nums[left] {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
            } else {
                // Right side
                if target < nums[mid] {
                    right = mid;
                } else {
                    if target <= nums[right - 1] {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
