// Time taken: 16:11, 17:01 -> Acc
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut left = node.borrow_mut().left.take();
            let mut right = node.borrow_mut().right.take();

            Self::flatten(&mut left);
            Self::flatten(&mut right);

            if let Some(left_node) = left.clone() {
                Self::append(left_node, right);
                node.borrow_mut().right = left;
            } else {
                node.borrow_mut().right = right;
            }
        }
    }

    pub fn append(mut left: Rc<RefCell<TreeNode>>, right: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = left.clone().borrow().right.clone() {
            left = node;
        }
        left.borrow_mut().right = right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
