// Time taken: 17:40, 17:50, 17:55 -> Wrong, 18:03 -> Acc, 18:22 -> Optimized

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let k = k as usize;
        let mut result = Vec::new();

        queue.push_back(nums[0]);
        for idx in 1..k {
            while !queue.is_empty() && *queue.back().unwrap() < nums[idx] {
                queue.pop_back();
            }
            queue.push_back(nums[idx]);
        }

        let mut left = 0;
        let mut right = k - 1;
        while right < nums.len() {
            let val = queue.front().unwrap().clone();
            result.push(val);
            if nums[left] == val {
                queue.pop_front();
            }

            left += 1;
            right += 1;

            if right < nums.len() {
                while !queue.is_empty() && *queue.back().unwrap() < nums[right] {
                    queue.pop_back();
                }
                queue.push_back(nums[right]);
            }
        }

        return result;
    }

    // pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let mut result = Vec::new();
    //     let k = k as usize;
    //     let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    //     let mut stack = Vec::new();

    //     for idx in 0..(k - 1) {
    //         max_heap.push(nums[idx]);
    //     }

    //     let mut left = 0;
    //     let mut right = k - 1;

    //     while right < nums.len() {
    //         max_heap.push(nums[right]);
    //         result.push(max_heap.peek().unwrap().clone());
    //         Self::binary_insert(&mut stack, nums[left]);

    //         while !stack.is_empty() {
    //             if stack[stack.len() - 1] == max_heap.peek().unwrap().clone() {
    //                 stack.pop();
    //                 max_heap.pop();
    //             } else {
    //                 break;
    //             }
    //         }

    //         left += 1;
    //         right += 1;
    //     }

    //     return result;
    // }

    // pub fn binary_insert(arr: &mut Vec<i32>, target: i32) {
    //     if arr.is_empty() {
    //         arr.push(target);
    //         return;
    //     }

    //     let mut left = 0;
    //     let mut right = arr.len();

    //     while left < right {
    //         let mid = left + ((right - left) / 2);
    //         if arr[mid] < target {
    //             left = mid + 1;
    //         } else {
    //             right = mid;
    //         }
    //     }

    //     arr.insert(left, target);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            [3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), [1]);
        assert_eq!(
            Solution::max_sliding_window(
                vec![-6, -10, -7, -1, -9, 9, -8, -4, 10, -5, 2, 9, 0, -7, 7, 4, -2, -10, 8, 7],
                7
            ),
            [9, 9, 10, 10, 10, 10, 10, 10, 10, 9, 9, 9, 8, 8]
        );
    }
}
