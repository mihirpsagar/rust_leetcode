// Time taken: 17:06, 17:36 -> Wrong, 17:46 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = nums[0];
        let mut right = nums[0];

        loop {
            left = nums[left as usize];
            right = nums[nums[right as usize] as usize];
            if left == right {
                break;
            }
        }

        let mut idx1 = nums[0];
        let mut idx2 = left;
        while idx1 != idx2 {
            idx1 = nums[idx1 as usize];
            idx2 = nums[idx2 as usize];
        }

        return idx1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![3, 3, 3, 3, 3]), 3);
        assert_eq!(
            Solution::find_duplicate(vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1]),
            9
        );
    }
}
