// Time taken: 12:39, 12:53 -> Acc

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let len = nums.len();
        let mut prefix = vec![0; len];
        let mut suffix = vec![0; len];

        let mut idx = 0;
        let mut val = 1;
        while idx < len {
            prefix[idx] = val;
            val *= nums[idx];
            idx += 1;
        }

        idx = len - 1;
        val = 1;
        loop {
            suffix[idx] = val;
            val *= nums[idx];
            if idx == 0 {
                break;
            } else {
                idx -= 1;
            }
        }

        idx = 0;
        while idx < len {
            result.push(prefix[idx] * suffix[idx]);
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
            Solution::product_except_self(vec![1, 2, 3, 4]),
            [24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            [0, 0, 9, 0, 0]
        );
    }
}
