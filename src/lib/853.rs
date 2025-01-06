// Time taken: 11:19, 12:46, 12:54 -> Wrong, 12:57 -> Wrong, 13:04 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arr = Vec::new();
        let mut idx = 0;
        let len = position.len();

        while idx < len {
            let distance = target - position[idx];
            let time = distance as f64 / speed[idx] as f64;

            arr.push((position[idx], time));
            idx += 1;
        }

        Self::quick_sort_rev(&mut arr, 0, len);

        let mut stack = Vec::new();
        stack.push(0);
        idx = 1;
        while idx < len {
            let prev_idx = stack[stack.len() - 1];
            if arr[idx].1 > arr[prev_idx].1 {
                stack.push(idx);
            }
            idx += 1;
        }

        return stack.len() as i32;
    }

    pub fn quick_sort_rev(mut arr: &mut Vec<(i32, f64)>, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mut left = start;
        let mut idx = start;
        let pivot = end - 1;

        while idx < end {
            if arr[idx].0 > arr[pivot].0 {
                let tmp = arr[left];
                arr[left] = arr[idx];
                arr[idx] = tmp;
                left += 1;
            }

            idx += 1;
        }

        let tmp = arr[left];
        arr[left] = arr[pivot];
        arr[pivot] = tmp;

        Self::quick_sort_rev(&mut arr, start, left);
        Self::quick_sort_rev(&mut arr, left + 1, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
        assert_eq!(
            Solution::car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]),
            6
        );
    }
}
