// Time taken: 20:50, 21:03 -> Acc
struct Solution {}

use std::cell::RefCell;
use std::cmp;
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::bst(&nums);
    }

    pub fn bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let (left, mid_right) = nums.split_at(nums.len() / 2);
        let (mid, right) = mid_right.split_first().unwrap();

        return Some(Rc::new(RefCell::new(TreeNode {
            val: *mid,
            left: Self::bst(&left),
            right: Self::bst(&right),
        })));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
