// Time taken: 20:30, 20:40 -> Acc
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => {
                return true;
            }
            Some(node) => {
                return Solution::is_symmetric_rec(&node.borrow().left, &node.borrow().right)
            }
        }
    }

    pub fn is_symmetric_rec(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => {
                return true;
            }
            (None, _) | (_, None) => {
                return false;
            }
            (Some(node1), Some(node2)) => {
                if node1.borrow().val != node2.borrow().val {
                    return false;
                }

                return Solution::is_symmetric_rec(&node1.borrow().left, &node2.borrow().right)
                    && Solution::is_symmetric_rec(&node1.borrow().right, &node2.borrow().left);
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
