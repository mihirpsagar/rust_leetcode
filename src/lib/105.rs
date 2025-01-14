// Time taken: 20:46, 21:01 -> Acc

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (idx, num) in inorder.iter().enumerate() {
            map.insert(num, idx);
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        let mut idx = 1;
        while idx < preorder.len() {
            let mut curr = root.clone();
            let curr_idx = map.get(&preorder[idx]).unwrap();
            let mut direction = Vec::new();

            while let Some(node) = curr {
                let node = node.borrow();
                let node_idx = map.get(&node.val).unwrap();
                if curr_idx < node_idx {
                    direction.push('L');
                    curr = node.left.clone();
                } else {
                    direction.push('R');
                    curr = node.right.clone();
                }
            }

            Self::insert_into_tree(root.clone(), direction, preorder[idx]);

            idx += 1;
        }

        return root;
    }

    pub fn insert_into_tree(root: Option<Rc<RefCell<TreeNode>>>, direction: Vec<char>, val: i32) {
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let mut curr = root;
        for ch in direction {
            let node = curr.unwrap();

            if ch == 'L' {
                if node.borrow().left.is_none() {
                    node.borrow_mut().left = new_node;
                    return;
                } else {
                    curr = node.borrow().left.clone();
                }
            } else {
                if node.borrow().right.is_none() {
                    node.borrow_mut().right = new_node;
                    return;
                } else {
                    curr = node.borrow().right.clone();
                }
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
