// Time taken: 11:44, 11:47 -> Wrong, 11:55 -> Acc

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let k = k as usize;
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for (key, val) in map {
            if min_heap.len() < k {
                min_heap.push(Reverse((val, key)));
            } else {
                if let Some(top_val) = min_heap.peek() {
                    if top_val.0 .0 < val {
                        min_heap.pop();
                        min_heap.push(Reverse((val, key)));
                    }
                }
            }
        }

        while let Some(val) = min_heap.pop() {
            result.push(val.0 .1);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), [1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), [1]);
    }
}
