// Time taken: 20:30, 21:13 -> Acc
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
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = (Vec::new(), 0);

        let mut stack = vec![];
        let mut curr = root;
        let mut prev = (None, 0);

        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node_val = node.borrow().val;
                if let Some(prev_val) = prev.0 {
                    if node_val == prev_val {
                        prev.1 += 1;
                    } else {
                        match prev.1.cmp(&result.1) {
                            Ordering::Equal => {
                                result.0.push(prev_val);
                            }
                            Ordering::Greater => {
                                result.0 = vec![prev_val];
                                result.1 = prev.1;
                            }
                            _ => {}
                        }
                        prev.0 = Some(node_val);
                        prev.1 = 1;
                    }
                } else {
                    prev = (Some(node_val), 1);
                }
                curr = node.borrow().right.clone();
            }
        }

        match prev.1.cmp(&result.1) {
            Ordering::Equal => {
                result.0.push(prev.0.unwrap());
            }
            Ordering::Greater => {
                result.0 = vec![prev.0.unwrap()];
                result.1 = prev.1;
            }
            _ => {}
        }

        return result.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
