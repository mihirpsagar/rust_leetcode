// Time taken: 23:55, 00:02, 00:06 -> Acc
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);

        while !queue.is_empty() {
            let mut val = 0.0_f64;
            let mut count: usize = 0;
            let mut queue2 = VecDeque::new();

            while let Some(node) = queue.pop_front() {
                if let Some(node) = node {
                    let node = node.borrow();
                    val += node.val as f64;
                    count += 1;
                    queue2.push_back(node.left.clone());
                    queue2.push_back(node.right.clone());
                }
            }

            queue = queue2;
            if count != 0 {
                result.push(val / count as f64);
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
