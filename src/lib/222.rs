// Time taken: 22:41, 22:57 -> Acc, 23:46 -> Optimized
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let left_len = Self::get_depth(&root, true);
        let right_len = Self::get_depth(&root, false);

        if left_len == right_len {
            return (1 << left_len) - 1;
        }

        let mut left = 1 << right_len;
        let mut right = (1 << left_len) - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::is_present(mid, 1 << (right_len - 1), &root) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left - 1;
    }

    fn is_present(
        search_index: i32,
        curr_level: i32,
        node: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match node {
            None => {
                return false;
            }
            Some(inner_node) => {
                if curr_level == 0 {
                    // At last level
                    return true;
                } else {
                    if search_index & curr_level == 0 {
                        // Belongs in left side
                        return Self::is_present(
                            search_index,
                            curr_level >> 1,
                            &inner_node.borrow().left,
                        );
                    } else {
                        // Belongs in right side
                        return Self::is_present(
                            search_index,
                            curr_level >> 1,
                            &inner_node.borrow().right,
                        );
                    }
                }
            }
        }
    }

    fn get_depth(node: &Option<Rc<RefCell<TreeNode>>>, check_left: bool) -> i32 {
        match node {
            None => {
                return 0;
            }
            Some(node) => {
                if check_left {
                    return 1 + Self::get_depth(&node.borrow().left, check_left);
                } else {
                    return 1 + Self::get_depth(&node.borrow().right, check_left);
                }
            }
        }
    }
    // pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut result = 0;
    //     let mut stack = vec![root];

    //     while let Some(Some(node)) = stack.pop() {
    //         result += 1;
    //         if node.borrow().left.is_some() {
    //             stack.push(node.borrow().left.clone());
    //             if node.borrow().right.is_some() {
    //                 stack.push(node.borrow().right.clone());
    //             }
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
