// Time taken: 01:18, 01:26 -> Acc
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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                continue;
            }

            let mut sum = 0;
            if let Some(left) = node.left.clone() {
                sum += left.borrow().val;
                queue.push_back(left);
            }
            if let Some(right) = node.right.clone() {
                sum += right.borrow().val;
                queue.push_back(right);
            }
            if sum != node.val {
                return false;
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
