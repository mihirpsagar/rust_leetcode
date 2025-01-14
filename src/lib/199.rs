// Time taken: 13:18, 13:21, 13:29 -> Acc
mod dsa;

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue1 = VecDeque::new();
        let mut queue2 = VecDeque::new();

        queue1.push_back(root);
        while !queue1.is_empty() || !queue2.is_empty() {
            if queue1.is_empty() {
                let tmp = queue1;
                queue1 = queue2;
                queue2 = tmp;
            }

            let mut last = None;
            while let Some(node) = queue1.pop_front() {
                if node.is_none() {
                    continue;
                }

                let node = node.unwrap();
                last = Some(node.borrow().val);

                if node.borrow().left.is_some() {
                    queue2.push_back(node.borrow().left.clone());
                }

                if node.borrow().right.is_some() {
                    queue2.push_back(node.borrow().right.clone());
                }
            }

            if let Some(val) = last {
                result.push(val);
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
