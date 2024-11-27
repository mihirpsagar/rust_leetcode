// Time taken: 21:40, 21:46 -> Wrong, 21:50 -> Wrong, 21:55 -> Acc

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    pub fn is_sub_path(
        mut head: Option<Box<ListNode>>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut arr = Vec::new();
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(inner_node) = node.clone() {
                let inner_node = inner_node.borrow();
                if inner_node.left.is_some() {
                    queue.push_back(inner_node.left.clone());
                }
                if inner_node.right.is_some() {
                    queue.push_back(inner_node.right.clone());
                }

                if Self::is_path_rec(&arr, node, 0) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn is_path_rec(arr: &Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>, idx: usize) -> bool {
        if idx >= arr.len() {
            return true;
        }

        if let Some(node) = node {
            let node = node.borrow();
            if node.val == arr[idx] {
                return Self::is_path_rec(&arr, node.left.clone(), idx + 1)
                    || Self::is_path_rec(&arr, node.right.clone(), idx + 1);
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
