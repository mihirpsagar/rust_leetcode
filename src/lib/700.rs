// Time taken: 14:03, 14:06 -> Acc
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
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let node = node.borrow();
            match node.val.cmp(&val) {
                Ordering::Equal => {
                    return root;
                }
                Ordering::Less => {
                    return Self::search_bst(node.right.clone(), val);
                }
                Ordering::Greater => {
                    return Self::search_bst(node.left.clone(), val);
                }
            }
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
