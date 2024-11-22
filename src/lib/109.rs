// Time taken: 15:59, 16:09 -> Acc
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
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut arr = Vec::new();

        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        return Self::generate_bst(&arr, 0, arr.len());
    }

    pub fn generate_bst(arr: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            return None;
        }

        let mid = start + (end - start) / 2;
        let bst = Rc::new(RefCell::new(TreeNode::new(arr[mid])));

        bst.borrow_mut().left = Self::generate_bst(&arr, start, mid);
        bst.borrow_mut().right = Self::generate_bst(&arr, mid + 1, end);

        return Some(bst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
