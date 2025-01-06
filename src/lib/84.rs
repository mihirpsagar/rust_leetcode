// Time taken: 13:14, 13:46 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;

        let mut idx = 0;
        while idx < heights.len() {
            if stack.is_empty() {
                stack.push((idx, heights[idx]));
            } else {
                let mut prev_idx = idx;
                while !stack.is_empty() && stack[stack.len() - 1].1 > heights[idx] {
                    let prev = stack.pop().unwrap();
                    let area = prev.1 * (idx - prev.0) as i32;
                    result = std::cmp::max(result, area);
                    prev_idx = prev.0;
                }

                stack.push((prev_idx, heights[idx]));
            }

            idx += 1;
        }

        while let Some(val) = stack.pop() {
            let area = val.1 * (idx - val.0) as i32;
            result = std::cmp::max(result, area);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
