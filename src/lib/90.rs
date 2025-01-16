// Time taken: 22:25, 22:31 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();
        Self::subset_rec(&nums, 0, &mut Vec::new(), &mut result);
        return result;
    }

    pub fn subset_rec(
        nums: &Vec<i32>,
        idx: usize,
        mut curr: &mut Vec<i32>,
        mut result: &mut Vec<Vec<i32>>,
    ) {
        // Base condition
        if idx >= nums.len() {
            result.push(curr.clone());
            return;
        }

        // Include idx in current array
        curr.push(nums[idx]);
        Self::subset_rec(&nums, idx + 1, &mut curr, &mut result);
        curr.pop();

        // Don't include idx in current array
        let mut next_idx = idx + 1;
        while next_idx < nums.len() && nums[next_idx] == nums[next_idx - 1] {
            next_idx += 1;
        }
        Self::subset_rec(&nums, next_idx, &mut curr, &mut result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![1, 2, 2],
                vec![1, 2],
                vec![1],
                vec![2, 2],
                vec![2],
                vec![]
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![0], vec![]]);
    }
}
