// Time taken: 13:45, 13:50 -> Acc
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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::rec_evaluate_tree(root.unwrap());
    }

    pub fn rec_evaluate_tree(node: Rc<RefCell<TreeNode>>) -> bool {
        let node = node.borrow();
        if node.val == 0 {
            return false;
        }
        if node.val == 1 {
            return true;
        }
        let left = Self::rec_evaluate_tree(node.left.clone().unwrap());
        let right = Self::rec_evaluate_tree(node.right.clone().unwrap());

        if node.val == 2 {
            return left || right;
        }

        return left && right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
