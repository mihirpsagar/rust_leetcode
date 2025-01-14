// Time taken: 14:09, 14:20, 14:24 -> Wrong, 16:31 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let k = k as usize;
        Self::kth_smallest_rec(root, &mut max_heap, k);
        return max_heap.pop().unwrap();
    }

    pub fn kth_smallest_rec(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut max_heap: &mut BinaryHeap<i32>,
        k: usize,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            let mut skip_right = false;
            if max_heap.len() < k {
                max_heap.push(node.val);
            } else {
                if *max_heap.peek().unwrap() > node.val {
                    max_heap.pop();
                    max_heap.push(node.val);
                } else {
                    skip_right = true;
                }
            }

            if node.left.is_some() {
                Self::kth_smallest_rec(node.left.clone(), &mut max_heap, k);
            }

            if !skip_right && node.right.is_some() {
                Self::kth_smallest_rec(node.right.clone(), &mut max_heap, k);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
