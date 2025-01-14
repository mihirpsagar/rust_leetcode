// Time taken: 21:22, 21:35, 21:38 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut set = HashSet::new();
        let mut result = Vec::new();
        let mut curr = Vec::new();
        candidates.sort();

        Self::dfs(&candidates, 0, 0, target, &mut curr, &mut set);

        for val in set {
            result.push(val);
        }

        return result;
    }

    fn dfs(
        candidates: &Vec<i32>,
        idx: usize,
        sum: i32,
        target: i32,
        mut curr: &mut Vec<i32>,
        mut set: &mut HashSet<Vec<i32>>,
    ) {
        // Exit conditions
        if !curr.is_empty() && sum == target {
            set.insert(curr.clone());
            return;
        }
        if idx >= candidates.len() || sum + candidates[idx] > target {
            return;
        }

        // Push value to curr & continue with same number
        curr.push(candidates[idx]);
        Self::dfs(
            &candidates,
            idx,
            sum + candidates[idx],
            target,
            &mut curr,
            &mut set,
        );

        // Push value to curr & Continue with next number
        Self::dfs(
            &candidates,
            idx + 1,
            sum + candidates[idx],
            target,
            &mut curr,
            &mut set,
        );
        curr.pop();

        // Don't push value & Continue with next number
        Self::dfs(&candidates, idx + 1, sum, target, &mut curr, &mut set);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(Solution::combination_sum(vec![2], 1).is_empty(), true);
    }
}
