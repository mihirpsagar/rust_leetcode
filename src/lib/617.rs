// Time taken: 22:38, 23:04 -> Acc
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut new_root = None;

        Self::merge_tree_rec(&root1, &root2, &mut new_root);

        return new_root;
    }

    fn merge_tree_rec<'a>(
        node1: &'a Option<Rc<RefCell<TreeNode>>>,
        node2: &'a Option<Rc<RefCell<TreeNode>>>,
        mut node3: &'a mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (node1, node2) {
            (None, None) => {
                return;
            }
            (Some(n1), None) | (None, Some(n1)) => {
                *node3 = Some(n1.clone());
            }
            (Some(n1), Some(n2)) => {
                let n1 = n1.borrow();
                let n2 = n2.borrow();
                *node3 = Some(Rc::new(RefCell::new(TreeNode {
                    val: n1.val + n2.val,
                    left: None,
                    right: None,
                })));

                Self::merge_tree_rec(
                    &n1.left,
                    &n2.left,
                    &mut node3.as_mut().unwrap().borrow_mut().left,
                );
                Self::merge_tree_rec(
                    &n1.right,
                    &n2.right,
                    &mut node3.as_mut().unwrap().borrow_mut().right,
                );
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
