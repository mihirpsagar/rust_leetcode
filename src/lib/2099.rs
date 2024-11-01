use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

// Time taken: 16:45, 16:52, 16:58, 17:31, 17:44 -> Acc, 18:28 -> Acc

struct Pair(i32, usize);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 != other.0 {
            return self.0.cmp(&other.0);
        }
        return self.1.cmp(&other.1);
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

struct Solution {}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut min_heap: BinaryHeap<Reverse<Pair>> = BinaryHeap::new();
        let k = k as usize;
        let mut tmp = Vec::new();
        let mut result = Vec::new();

        for (idx, &num) in nums.iter().enumerate() {
            if min_heap.len() < k {
                min_heap.push(Reverse(Pair(num, idx)));
            } else {
                if min_heap.peek().unwrap().0 .0 < num {
                    min_heap.pop();
                    min_heap.push(Reverse(Pair(num, idx)));
                }
            }
        }

        while let Some(val) = min_heap.pop() {
            tmp.push(val.0);
        }

        tmp.sort_by(|a, b| {
            return a.1.cmp(&b.1);
        });

        for val in tmp {
            result.push(val.0);
        }

        return result;
    }
    // pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let len = nums.len();
    //     let k = k as usize;
    //     let mut sorted_nums = nums.clone();
    //     let mut result = Vec::new();
    //     let mut valid = Vec::new();

    //     Self::quick_sort(&mut sorted_nums, 0, len);

    //     let mut idx = len - k;
    //     for _ in 0..k {
    //         valid.push(sorted_nums[idx]);
    //         idx += 1;
    //     }

    //     for num in nums {
    //         if Self::binary_search_and_remove(&mut valid, num) {
    //             result.push(num);
    //         }
    //     }

    //     return result;
    // }

    // pub fn binary_search_and_remove(arr: &mut Vec<i32>, target: i32) -> bool {
    //     let mut left = 0;
    //     let mut right = arr.len();

    //     while left < right {
    //         let mid = left + (right - left) / 2;
    //         if arr[mid] == target {
    //             arr.remove(mid);
    //             return true;
    //         } else if arr[mid] < target {
    //             left = mid + 1;
    //         } else {
    //             right = mid;
    //         }
    //     }

    //     return false;
    // }

    // pub fn quick_sort(mut arr: &mut Vec<i32>, start: usize, end: usize) {
    //     if start >= end {
    //         return;
    //     }

    //     let mut left = start;
    //     let pivot = end - 1;
    //     let mut idx = start;

    //     while idx < end {
    //         if arr[idx] < arr[pivot] {
    //             let tmp = arr[left];
    //             arr[left] = arr[idx];
    //             arr[idx] = tmp;
    //             left += 1;
    //         }
    //         idx += 1;
    //     }

    //     let tmp = arr[left];
    //     arr[left] = arr[pivot];
    //     arr[pivot] = tmp;

    //     Self::quick_sort(&mut arr, start, left);
    //     Self::quick_sort(&mut arr, left + 1, end);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), [3, 3]);
        assert_eq!(Solution::max_subsequence(vec![-1, -2, 3, 4], 3), [-1, 3, 4]);
        assert_eq!(Solution::max_subsequence(vec![3, 4, 3, 3], 2), [3, 4]);
    }
}
