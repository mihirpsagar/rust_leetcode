// Time taken: 20:01, 20:09 -> Wrong, 20:23 -> TLE, 20:27 -> TLE, 20:33 -> TLE, 20:44 -> Wrong, 20:49 -> Acc
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
    // pub fn is_subtree(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     sub_root: Option<Rc<RefCell<TreeNode>>>,
    // ) -> bool {
    //     if sub_root.is_none() {
    //         return true;
    //     }

    //     if root.is_none() {
    //         return false;
    //     }

    //     let mut root_str = String::new();
    //     let mut sub_root_str = String::new();
    //     Self::convert_bst_string(&root, &mut root_str);
    //     Self::convert_bst_string(&sub_root, &mut sub_root_str);

    //     let lps = Self::calculate_lps(&sub_root_str);

    //     return Self::kmp_search(&root_str, &sub_root_str, &lps);
    // }

    // fn kmp_search(str: &String, sub_str: &String, lps: &Vec<usize>) -> bool {
    //     let str = str.chars().collect::<Vec<char>>();
    //     let sub_str = sub_str.chars().collect::<Vec<char>>();
    //     let str_len = str.len();
    //     let sub_str_len = sub_str.len();
    //     let mut i = 0;
    //     let mut j = 0;

    //     while i < str_len && j < sub_str_len && (str_len - i) >= (sub_str_len - j) {
    //         if str[i] == sub_str[j] {
    //             i += 1;
    //             j += 1;

    //             if j == sub_str_len {
    //                 return true;
    //             }
    //         } else {
    //             if j != 0 {
    //                 j = lps[j - 1];
    //             } else {
    //                 i += 1;
    //             }
    //         }
    //     }

    //     return false;
    // }

    // fn calculate_lps(str: &String) -> Vec<usize> {
    //     let mut lps = Vec::new();
    //     let str = str.chars().collect::<Vec<char>>();
    //     let mut len = 0;
    //     lps.push(0);
    //     let mut idx = 1;

    //     while idx < str.len() {
    //         if str[idx] == str[len] {
    //             len += 1;
    //             lps.push(len);
    //             idx += 1;
    //         } else {
    //             if len != 0 {
    //                 len = lps[len - 1];
    //             } else {
    //                 lps.push(0);
    //                 idx += 1;
    //             }
    //         }
    //     }

    //     return lps;
    // }

    // fn convert_bst_string(node: &Option<Rc<RefCell<TreeNode>>>, mut str: &mut String) {
    //     match node {
    //         None => {
    //             str.push_str(&",#");
    //         }
    //         Some(node) => {
    //             let node = node.borrow();
    //             str.push_str(&format!(",{}", node.val.to_string()));
    //             Self::convert_bst_string(&node.left, &mut str);
    //             Self::convert_bst_string(&node.right, &mut str);
    //         }
    //     }
    // }

    pub fn is_subtree_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(node1) = root {
            if let Some(node2) = sub_root {
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                if node1.val == node2.val {
                    if Self::is_subtree_rec(&node1.left, &node2.left) {
                        if Self::is_subtree_rec(&node1.right, &node2.right) {
                            return true;
                        }
                    }
                }

                return false;
            } else {
                return false;
            }
        } else {
            if sub_root.is_some() {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if sub_root.is_none() {
            return true;
        }

        if root.is_none() {
            return false;
        }

        if Self::is_subtree_rec(&root, &sub_root) {
            return true;
        }

        return Self::is_subtree(
            root.as_ref().unwrap().borrow().left.clone(),
            sub_root.clone(),
        ) || Self::is_subtree(
            root.as_ref().unwrap().borrow().right.clone(),
            sub_root.clone(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        Solution::is_subtree(None, None);
    }
}
