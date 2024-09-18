// Time taken: 21:54, 22:09 -> Acc
struct Solution {}

use std::cell::RefCell;
use std::cmp::{self, Ordering};
use std::collections::VecDeque;
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut result = false;

        match root {
            None => {
                return false;
            }
            Some(node) => {
                let mut left_vec = Self::has_path_sum_rec(&node.borrow().left);
                let mut right_vec = Self::has_path_sum_rec(&node.borrow().right);
                left_vec.append(&mut right_vec);

                for i in 0..left_vec.len() {
                    if (left_vec[i] + node.borrow().val) == target_sum {
                        return true;
                    }
                }

                if left_vec.len() == 0 && node.borrow().val == target_sum {
                    return true;
                }
            }
        }

        return result;
    }

    pub fn has_path_sum_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        match node {
            None => {
                return result;
            }
            Some(inner_node) => {
                let mut left_vec = Self::has_path_sum_rec(&inner_node.borrow().left);
                let mut right_vec = Self::has_path_sum_rec(&inner_node.borrow().right);

                result.append(&mut left_vec);
                result.append(&mut right_vec);

                for i in 0..result.len() {
                    result[i] += inner_node.borrow().val;
                }

                if result.len() == 0 {
                    result.push(inner_node.borrow().val);
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
