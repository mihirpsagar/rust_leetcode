// Time taken: 13:45, 13:52 -> Wrong, 14:02 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    i32,
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = true;
        let mut checks = Vec::new();
        Self::is_valid(root, &mut result, &mut checks);
        return result;
    }

    pub fn is_valid(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut result: &mut bool,
        mut checks: &mut Vec<(char, i32)>,
    ) {
        if !*result {
            return;
        }

        if let Some(node) = node {
            let node = node.borrow();

            for check in checks.iter() {
                if check.0 == 'L' {
                    if node.val >= check.1 {
                        *result = false;
                        return;
                    }
                } else {
                    if node.val <= check.1 {
                        *result = false;
                        return;
                    }
                }
            }

            if node.left.is_some() {
                checks.push(('L', node.val));
                Self::is_valid(node.left.clone(), &mut result, &mut checks);
                checks.pop();
            }

            if node.right.is_some() {
                checks.push(('R', node.val));
                Self::is_valid(node.right.clone(), &mut result, &mut checks);
                checks.pop();
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
