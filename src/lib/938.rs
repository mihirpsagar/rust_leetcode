// Time taken: 12:15, 12:19 -> Acc, 12:25 -> Optimized

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let node = node.borrow();
            if node.val < low {
                return Self::range_sum_bst(node.right.clone(), low, high);
            } else if node.val > high {
                return Self::range_sum_bst(node.left.clone(), low, high);
            } else {
                sum += node.val;
                sum += Self::range_sum_bst(node.left.clone(), low, high);
                sum += Self::range_sum_bst(node.right.clone(), low, high);
            }
        }

        return sum;
    }

    // pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    //     let mut stack = Vec::new();
    //     let mut curr = root;
    //     let mut sum = 0;

    //     while !stack.is_empty() || curr.is_some() {
    //         while let Some(node) = curr {
    //             stack.push(node.clone());
    //             curr = node.borrow().left.clone();
    //         }

    //         if let Some(node) = stack.pop() {
    //             let node = node.borrow();

    //             if node.val >= low && node.val <= high {
    //                 sum += node.val;
    //             } else {
    //                 if node.val > high {
    //                     break;
    //                 }
    //             }
    //             curr = node.right.clone();
    //         }
    //     }

    //     return sum;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
