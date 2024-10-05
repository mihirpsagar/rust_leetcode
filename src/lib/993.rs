// Time taken: 23:03, 23:12, 23:16 -> Acc
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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 1, 0));
        let mut found = (None, None);

        while let Some((node, depth, parent)) = queue.pop_front() {
            let node = node.borrow();

            if node.val == x {
                found.0 = Some((depth, parent));
            } else if node.val == y {
                found.1 = Some((depth, parent));
            }

            if let Some((depth1, _)) = found.0 {
                if depth > depth1 {
                    break;
                }
            } else if let Some((depth2, _)) = found.1 {
                if depth > depth2 {
                    break;
                }
            }

            if let Some(left) = node.left.clone() {
                queue.push_back((left, depth + 1, node.val));
            }

            if let Some(right) = node.right.clone() {
                queue.push_back((right, depth + 1, node.val));
            }
        }

        if found.0.is_none() || found.1.is_none() {
            return false;
        }

        let found1 = found.0.unwrap();
        let found2 = found.1.unwrap();
        return found1.0 == found2.0 && found1.1 != found2.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
