// Time taken: 16:04, 16:11 -> Wrong, 16:31 -> Wrong, 16:33 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

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

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = None;
        let _ = Self::max_path_sum_rec(root, &mut result);
        return result.unwrap();
    }

    pub fn max_path_sum_rec(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut result: &mut Option<i32>,
    ) -> Option<i32> {
        if let Some(node) = node {
            let node = node.borrow();

            let mut left_sum = None;
            if node.left.is_some() {
                left_sum = Self::max_path_sum_rec(node.left.clone(), &mut result);
            }

            let mut right_sum = None;
            if node.right.is_some() {
                right_sum = Self::max_path_sum_rec(node.right.clone(), &mut result);
            }

            let mut max = None;

            match (left_sum, right_sum) {
                (None, None) => {
                    max = Some(node.val);
                    if result.is_none() {
                        *result = Some(node.val);
                    } else {
                        let curr_max = result.unwrap();
                        *result = Some(std::cmp::max(node.val, curr_max));
                    }
                }

                (left_val, None) => {
                    max = Some(std::cmp::max(node.val + left_val.unwrap(), node.val));
                    let mut curr_max = node.val;

                    if result.is_some() {
                        curr_max = std::cmp::max(curr_max, result.unwrap());
                    }

                    curr_max = std::cmp::max(curr_max, left_val.unwrap());
                    curr_max = std::cmp::max(curr_max, left_val.unwrap() + node.val);
                    *result = Some(curr_max);
                }

                (None, right_val) => {
                    max = Some(std::cmp::max(node.val + right_val.unwrap(), node.val));
                    let mut curr_max = node.val;

                    if result.is_some() {
                        curr_max = std::cmp::max(curr_max, result.unwrap());
                    }

                    curr_max = std::cmp::max(curr_max, right_val.unwrap());
                    curr_max = std::cmp::max(curr_max, right_val.unwrap() + node.val);
                    *result = Some(curr_max);
                }

                (left_val, right_val) => {
                    let tmp = std::cmp::max(left_val.unwrap(), right_val.unwrap());
                    max = Some(std::cmp::max(node.val + tmp, node.val));

                    let mut curr_max = node.val;

                    if result.is_some() {
                        curr_max = std::cmp::max(curr_max, result.unwrap());
                    }

                    curr_max = std::cmp::max(curr_max, left_val.unwrap());
                    curr_max = std::cmp::max(curr_max, right_val.unwrap());
                    curr_max = std::cmp::max(curr_max, left_val.unwrap() + node.val);
                    curr_max = std::cmp::max(curr_max, right_val.unwrap() + node.val);
                    curr_max =
                        std::cmp::max(curr_max, left_val.unwrap() + right_val.unwrap() + node.val);
                    *result = Some(curr_max);
                }
            }

            return max;
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
