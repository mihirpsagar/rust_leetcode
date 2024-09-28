// Time taken: 00:51, 00:56, 01:04 -> Acc
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
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MAX;
        let mut prev = None;
        let mut stack = Vec::new();
        let mut curr = root;

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node = node.borrow();
                if let Some(prev_val) = prev {
                    if node.val - prev_val < result {
                        result = node.val - prev_val;
                    }
                    prev = Some(node.val);
                } else {
                    prev = Some(node.val);
                }
                curr = node.right.clone();
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
