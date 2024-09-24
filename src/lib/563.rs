// Time taken: 19:28, 19:40 -> Acc
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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut sum: &mut i32) -> i32 {
            if let Some(node) = node {
                let left = dfs(&node.borrow().left, &mut sum);
                let right = dfs(&node.borrow().right, &mut sum);

                *sum += left.abs_diff(right) as i32;

                return (left + right + node.borrow().val);
            } else {
                return 0;
            }
        }

        let mut result = 0;
        dfs(&root, &mut result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
