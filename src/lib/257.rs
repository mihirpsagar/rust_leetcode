// Time taken: 15:00, 15:15 -> Acc
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Vec::new();

        if root.is_none() {
            return result;
        } else {
            let mut stack = Vec::new();
            Self::binary_tree_paths_rc(&root, &mut stack, &mut result);
        }

        return result;
    }

    pub fn binary_tree_paths_rc(
        node: &Option<Rc<RefCell<TreeNode>>>,
        mut stack: &mut Vec<String>,
        mut result: &mut Vec<String>,
    ) {
        let node = node.as_ref().unwrap().borrow();
        stack.push(node.val.to_string());

        match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => {
                result.push(stack.join("->"));
            }
            (Some(_), None) => {
                Self::binary_tree_paths_rc(&node.left, &mut stack, &mut result);
            }
            (None, Some(_)) => {
                Self::binary_tree_paths_rc(&node.right, &mut stack, &mut result);
            }
            (Some(_), Some(_)) => {
                Self::binary_tree_paths_rc(&node.left, &mut stack, &mut result);
                Self::binary_tree_paths_rc(&node.right, &mut stack, &mut result);
            }
        }
        stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
