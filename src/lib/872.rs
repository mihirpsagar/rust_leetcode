// Time taken: 23:15, 23:21, 23:28 -> Acc
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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaf1 = Vec::new();
        let mut leaf2 = Vec::new();

        let mut stack = Vec::new();
        let mut curr = root1;

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    leaf1.push(node.val);
                }
                curr = node.right.clone();
            }
        }

        curr = root2;
        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    leaf2.push(node.val);
                }
                curr = node.right.clone();
            }
        }

        if leaf1.len() != leaf2.len() {
            return false;
        }

        let mut idx = 0;
        while idx < leaf1.len() {
            if leaf1[idx] != leaf2[idx] {
                return false;
            }
            idx += 1;
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
