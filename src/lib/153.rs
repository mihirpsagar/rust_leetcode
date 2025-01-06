// Time taken: 20:44, 21:08 -> Wrong, 21:10 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[(mid + 1) % nums.len()] < nums[mid] {
                return nums[(mid + 1) % nums.len()];
            }
            if mid == 0 {
                if nums[nums.len() - 1] > nums[mid] {
                    return nums[mid];
                }
            } else {
                if nums[mid - 1] > nums[mid] {
                    return nums[mid];
                }
            }

            if nums[right - 1] < nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
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
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
