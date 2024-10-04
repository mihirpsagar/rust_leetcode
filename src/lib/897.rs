// Time taken: 23:09, 23:24 -> Acc
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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut stack = Vec::new();
        let mut nums = Vec::new();
        let mut curr = root;

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node = node.borrow();
                nums.push(node.val);
                curr = node.right.clone();
            }
        }

        let mut result = None;
        for num in nums.iter().rev() {
            let new_node = Rc::new(RefCell::new(TreeNode {
                val: *num,
                left: None,
                right: None,
            }));
            match result {
                None => {
                    result = Some(new_node);
                }
                Some(prev_node) => {
                    new_node.borrow_mut().right = Some(prev_node);
                    result = Some(new_node);
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
