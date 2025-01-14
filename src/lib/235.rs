// Time taken: 20:54, 21:12, 21:19 -> Wrong, 21:28 -> Wrong, 00:50 -> Acc

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p {
            return p;
        }

        if root == q {
            return q;
        }

        let mut p = p.unwrap().borrow().val;
        let mut q = q.unwrap().borrow().val;
        if p > q {
            let tmp = p;
            p = q;
            q = tmp;
        }

        let mut curr = root;
        while curr.is_some() {
            let node = curr.clone().unwrap();
            let val = node.borrow().val;
            if val == p || val == q || (val > p && val < q) {
                return curr;
            }
            if val < p {
                curr = node.borrow().right.clone();
            } else {
                curr = node.borrow().left.clone();
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
