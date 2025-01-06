// Time taken: 11:03, 11:15 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        let mut idx = 0;
        while idx < temperatures.len() {
            while !stack.is_empty()
                && temperatures[stack[stack.len() - 1] as usize] < temperatures[idx]
            {
                let prev_idx = stack[stack.len() - 1];
                result[prev_idx as usize] = idx as i32 - prev_idx;
                stack.pop();
            }

            stack.push(idx as i32);
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
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            [1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            [1, 1, 1, 0]
        );
        assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), [1, 1, 0]);
    }
}
