// Time taken: 21:00, 23:41 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = height.len();
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];

        let mut idx = 0;
        let mut max = 0;
        while idx < len {
            left_max[idx] = max;
            max = std::cmp::max(max, height[idx]);
            idx += 1;
        }

        idx = len - 1;
        max = 0;
        loop {
            right_max[idx] = max;
            max = std::cmp::max(max, height[idx]);
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        idx = 0;
        while idx < len {
            if height[idx] < left_max[idx] && height[idx] < right_max[idx] {
                result = result + (std::cmp::min(left_max[idx], right_max[idx]) - height[idx]);
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
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
