// Time taken: 00:54, 01:03, 01:19 -> Acc
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
    // pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    //         match node {
    //             None => {
    //                 return (0, 0);
    //             }
    //             Some(node) => {
    //                 let (left, left_max) = dfs(&node.borrow().left);
    //                 let (right, right_max) = dfs(&node.borrow().right);
    //                 let max = left_max.max(right_max).max(left + right);
    //                 if left < right {
    //                     return (left + 1, max);
    //                 } else {
    //                     return (right + 1, max);
    //                 }
    //             }
    //         }
    //     }

    //     return dfs(&root).1;
    // }
    // pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut max_so_far = 0;
    //     Self::dfs(root, &mut max_so_far);

    //     return max_so_far;
    // }

    // fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut max_so_far: &mut i32) -> (i32, i32) {
    //     match node {
    //         None => {
    //             return (-1, -1);
    //         }
    //         Some(node) => {
    //             let left = Self::dfs(node.borrow().left.clone(), &mut max_so_far);
    //             let right = Self::dfs(node.borrow().right.clone(), &mut max_so_far);
    //             let left_val = 1 + std::cmp::max(left.0, left.1);
    //             let right_val = 1 + std::cmp::max(right.0, right.1);
    //             if *max_so_far < left_val {
    //                 *max_so_far = left_val;
    //             }
    //             if *max_so_far < right_val {
    //                 *max_so_far = right_val;
    //             }
    //             if *max_so_far < (left_val + right_val) {
    //                 *max_so_far = left_val + right_val;
    //             }
    //             return (left_val, right_val);
    //         }
    //     }
    // }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let _ = Self::diameter_of_binary_tree_rec(root, &mut result);
        return result;
    }

    pub fn diameter_of_binary_tree_rec(
        node: Option<Rc<RefCell<TreeNode>>>,
        result: &mut i32,
    ) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let mut left_val = 0;
            let mut right_val = 0;
            if node.left.is_some() {
                left_val = 1 + Self::diameter_of_binary_tree_rec(node.left.clone(), result);
            }
            if node.right.is_some() {
                right_val = 1 + Self::diameter_of_binary_tree_rec(node.right.clone(), result);
            }

            if left_val > 0 || right_val > 0 {
                *result = std::cmp::max(*result, left_val + right_val);
            }

            return std::cmp::max(left_val, right_val);
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
