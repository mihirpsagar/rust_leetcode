use std::collections::HashSet;

// Time taken: 22:33, 22:40 -> Acc
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut hash_set = HashSet::new();
        let mut stack = Vec::new();
        stack.push(root);

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                if hash_set.contains(&(k - node.val)) {
                    return true;
                } else {
                    hash_set.insert(node.val);
                }
                stack.push(node.left.clone());
                stack.push(node.right.clone());
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
