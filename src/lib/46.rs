// Time taken: 22:11, 22:21, 23:32 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut idx = 0;
        while idx < nums.len() {
            Self::permute_rec(nums.clone(), idx, Vec::new(), &mut result);
            idx += 1;
        }
        return result;
    }

    fn permute_rec(
        mut arr: Vec<i32>,
        idx: usize,
        mut curr: Vec<i32>,
        mut result: &mut Vec<Vec<i32>>,
    ) {
        curr.push(arr[idx]);

        if arr.len() == 1 {
            result.push(curr.clone());
            return;
        }

        // Remove idx from arr and call with other indices
        arr.remove(idx);
        let mut next_idx = 0;
        while next_idx < arr.len() {
            Self::permute_rec(arr.clone(), next_idx, curr.clone(), &mut result);
            next_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }
}
