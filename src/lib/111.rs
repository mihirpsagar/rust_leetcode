// Time taken: 21:14, 21:19
struct Solution {}

use std::cell::RefCell;
use std::cmp::{self, Ordering};
use std::collections::VecDeque;
use std::rc::Rc;

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        let mut depth = 1;
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let curr_tree_breadth = queue.len();
            for _ in 0..curr_tree_breadth {
                let node = queue.pop_front().unwrap();
                let mut node_ref = node.borrow_mut();

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return depth;
                }

                if let Some(left_node) = node_ref.left.take() {
                    queue.push_back(left_node);
                }

                if let Some(right_node) = node_ref.right.take() {
                    queue.push_back(right_node);
                }
            }
            depth += 1;
        }

        unreachable!();
    }

    // pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     return Self::min_depth_rec(&root);
    // }

    // pub fn min_depth_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     match node {
    //         None => {
    //             return 0;
    //         }
    //         Some(inner_node) => {
    //             let left_depth = Self::min_depth_rec(&inner_node.borrow().left);
    //             let right_depth = Self::min_depth_rec(&inner_node.borrow().right);

    //             if left_depth == 0 || right_depth == 0 {
    //                 return cmp::max(left_depth, right_depth) + 1;
    //             } else {
    //                 return cmp::min(left_depth, right_depth) + 1;
    //             }
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
