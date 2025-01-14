// Time taken: 13:32, 13:35 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let k = k as usize;

        for num in nums {
            if min_heap.len() < k {
                min_heap.push(Reverse(num));
            } else {
                if min_heap.peek().unwrap().0 < num {
                    min_heap.pop();
                    min_heap.push(Reverse(num));
                }
            }
        }

        return min_heap.pop().unwrap().0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
