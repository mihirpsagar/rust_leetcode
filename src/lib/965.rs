// Time taken: 13:38, 13:44 -> Acc
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();

        if root.is_none() {
            return true;
        }
        let mut prev_val = root.clone().unwrap().borrow().val;

        queue.push_back(root.unwrap());
        while let Some(node) = queue.pop_front() {
            let node = node.borrow();

            if prev_val != node.val {
                return false;
            }

            if let Some(left_val) = node.left.clone() {
                queue.push_back(left_val);
            }

            if let Some(right_val) = node.right.clone() {
                queue.push_back(right_val);
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
