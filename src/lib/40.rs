// Time taken: 21:43, 21:49 -> TLE, 21:55, 22:08 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut curr = Vec::new();
        candidates.sort();

        Self::dfs(&candidates, 0, 0, target, &mut curr, &mut result);

        return result;
    }

    fn dfs(
        candidates: &Vec<i32>,
        idx: usize,
        sum: i32,
        target: i32,
        mut curr: &mut Vec<i32>,
        mut result: &mut Vec<Vec<i32>>,
    ) {
        // Exit case
        if !curr.is_empty() && sum == target {
            result.push(curr.clone());
            return;
        }
        if idx >= candidates.len() || sum > target {
            return;
        }

        // Include current value
        curr.push(candidates[idx]);
        Self::dfs(
            &candidates,
            idx + 1,
            sum + candidates[idx],
            target,
            &mut curr,
            &mut result,
        );

        // Don't include current value
        curr.pop();
        let mut next_idx = idx + 1;
        while next_idx < candidates.len() && candidates[next_idx] == candidates[next_idx - 1] {
            next_idx += 1;
        }

        Self::dfs(&candidates, next_idx, sum, target, &mut curr, &mut result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
