// Time taken: 13:15, 13:24 -> Acc
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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::dfs(root, vec![]);
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, path: Vec<i32>) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let mut new_path = path;
            let mut sum = 0;
            new_path.push(node.val);

            if node.left.is_none() && node.right.is_none() {
                let mut idx = 0;
                while idx < new_path.len() {
                    sum = (sum * 2) + new_path[idx];
                    idx += 1;
                }
                return sum;
            } else {
                return Self::dfs(node.left.clone(), new_path.clone())
                    + Self::dfs(node.right.clone(), new_path);
            }
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
