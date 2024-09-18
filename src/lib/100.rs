// Time taken: 20:25, 20:29 -> Acc
struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => {
                return true;
            }
            (None, _) => {
                return false;
            }
            (_, None) => {
                return false;
            }
            (Some(node1), Some(node2)) => {
                if node1.borrow().val != node2.borrow().val {
                    return false;
                }

                return Solution::is_same_tree(
                    node1.borrow().left.clone(),
                    node2.borrow().left.clone(),
                ) && Solution::is_same_tree(
                    node1.borrow().right.clone(),
                    node2.borrow().right.clone(),
                );
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
