// Time taken: 21:05, 21:11
struct Solution {}

use std::cell::RefCell;
use std::cmp::{self, Ordering};
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => {
                return true;
            }
            Some(node) => {
                let (left_res, left_depth) = Self::is_balanced_rec(&node.borrow().left);
                let (right_res, right_depth) = Self::is_balanced_rec(&node.borrow().right);

                return left_res && right_res && (left_depth.abs_diff(right_depth) < 2);
            }
        }
    }

    pub fn is_balanced_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, usize) {
        match node {
            None => {
                return (true, 0);
            }
            Some(inner_node) => {
                let (left_res, left_depth) = Self::is_balanced_rec(&inner_node.borrow().left);
                let (right_res, right_depth) = Self::is_balanced_rec(&inner_node.borrow().right);
                let balanced = left_res && right_res && (left_depth.abs_diff(right_depth) < 2);
                return (balanced, cmp::max(left_depth, right_depth) + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //[1,2,2,3,null,null,3,4,null,null,4]
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        Solution::is_balanced(root);
    }
}
