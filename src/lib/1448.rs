// Time taken: 13:36, 13:40 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    i32,
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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::dfs(root, &mut result, i32::MIN);
        return result;
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut result: &mut i32, max_so_far: i32) {
        if let Some(node) = node {
            let node = node.borrow();
            let mut max = max_so_far;
            if node.val >= max_so_far {
                *result += 1;
                max = node.val;
            }

            if node.left.is_some() {
                Self::dfs(node.left.clone(), &mut result, max);
            }

            if node.right.is_some() {
                Self::dfs(node.right.clone(), &mut result, max);
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
