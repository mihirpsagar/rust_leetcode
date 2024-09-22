// Time taken: 12:24, 12:35 -> Acc
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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => {
                return 0;
            }
            Some(node) => {
                return Self::sum_of_left_leaves_rec(&node.borrow().left, true)
                    + Self::sum_of_left_leaves_rec(&node.borrow().right, false);
            }
        }
    }

    pub fn sum_of_left_leaves_rec(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match node {
            None => {
                return 0;
            }
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() && is_left {
                    return node.borrow().val;
                }

                return Self::sum_of_left_leaves_rec(&node.borrow().left, true)
                    + Self::sum_of_left_leaves_rec(&node.borrow().right, false);
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
