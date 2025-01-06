// Time taken: 12:22, 12:35 -> Acc
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
    // pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     root.map(|node| {
    //         let node = node.borrow();
    //         let mut new_node = TreeNode {
    //             val: node.val,
    //             left: None,
    //             right: None,
    //         };
    //         new_node.left = Self::invert_tree(node.right.clone());
    //         new_node.right = Self::invert_tree(node.left.clone());
    //         Rc::new(RefCell::new(new_node))
    //     })
    // }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = Self::invert_tree(node.left.take());
            let right = Self::invert_tree(node.right.take());
            node.left = right;
            node.right = left;
            return root;
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
