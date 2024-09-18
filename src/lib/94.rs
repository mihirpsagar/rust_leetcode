// Time taken: 19:52, 20:03 -> Acc,
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack = Vec::new();
        let mut curr_node = root;

        while curr_node.is_some() || !stack.is_empty() {
            while let Some(node) = curr_node {
                stack.push(node.clone());
                curr_node = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                curr_node = node.borrow().right.clone();
            }
        }

        return result;
    }

    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut path: Vec<i32> = Vec::new();
    //     Solution::inorder_traversal_rec(&root, &mut path);

    //     return path;
    // }

    // pub fn inorder_traversal_rec(node: &Option<Rc<RefCell<TreeNode>>>, mut path: &mut Vec<i32>) {
    //     match node {
    //         None => {
    //             return;
    //         }
    //         Some(inner_node) => {
    //             Solution::inorder_traversal_rec(&inner_node.borrow().left, &mut path);
    //             path.push(inner_node.borrow().val);
    //             Solution::inorder_traversal_rec(&inner_node.borrow().right, &mut path);
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
