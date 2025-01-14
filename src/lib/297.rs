// Time taken: 13:27, 12:13 -> Acc
// mod dsa;

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

impl Solution {}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        return Self {};
    }

    fn build_string_rec(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut arr: &mut Vec<String>,
        mut direction: &mut String,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            let mut result_str = node.val.to_string();

            if !direction.is_empty() {
                result_str.push(',');
                result_str.push_str(&direction.clone());
            }
            arr.push(result_str);

            if node.left.is_some() {
                direction.push('L');
                Self::build_string_rec(node.left.clone(), &mut arr, &mut direction);
                direction.pop();
            }

            if node.right.is_some() {
                direction.push('R');
                Self::build_string_rec(node.right.clone(), &mut arr, &mut direction);
                direction.pop();
            }
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        }

        // Format: "1;2,L;3,R;4,RL;5,RR"
        let mut arr = Vec::new();
        let mut direction = String::new();

        // Step 1: Recursively pass the node and direction and create the array
        Self::build_string_rec(root, &mut arr, &mut direction);

        // Step 2: Join the arr with ";" and return the string
        return arr.join(";");

        // let mut arr = Vec::new();
        // let mut queue = VecDeque::new();
        // queue.push_back(root);

        // while !queue.is_empty() {
        //     let len = queue.len();
        //     let mut all_null = true;

        //     for node in queue.iter() {
        //         if node.is_some() {
        //             all_null = false;
        //             break;
        //         }
        //     }

        //     if all_null {
        //         break;
        //     }

        //     for _ in 0..len {
        //         let node = queue.pop_front().unwrap();
        //         if node.is_none() {
        //             arr.push(String::from("#,"));
        //             queue.push_back(None);
        //             queue.push_back(None);
        //         } else {
        //             let node = node.unwrap();
        //             let inner_node = node.borrow();
        //             arr.push(String::from(inner_node.val.to_string() + ","));
        //             queue.push_back(inner_node.left.clone());
        //             queue.push_back(inner_node.right.clone());
        //         }
        //     }
        // }

        // let mut result = Vec::new();
        // for str in arr {
        //     for ch in str.chars() {
        //         result.push(ch);
        //     }
        // }

        // result.pop();
        // return result.iter().collect();
    }

    fn create_node(node: Option<Rc<RefCell<TreeNode>>>, str: &str) {
        let arr = str.split(',').collect::<Vec<&str>>();
        let val = i32::from_str_radix(arr[0], 10).unwrap();
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));

        let mut curr = node;
        for ch in arr[1].chars() {
            let curr_node = curr.unwrap();

            if ch == 'L' {
                if curr_node.borrow().left.is_some() {
                    curr = curr_node.borrow().left.clone();
                } else {
                    curr_node.borrow_mut().left = new_node;
                    return;
                }
            } else {
                if curr_node.borrow().right.is_some() {
                    curr = curr_node.borrow().right.clone();
                } else {
                    curr_node.borrow_mut().right = new_node;
                    return;
                }
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let arr = data.split(';').collect::<Vec<&str>>();

        let val = i32::from_str_radix(arr[0].split(',').collect::<Vec<&str>>()[0], 10).unwrap();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(val))));

        let mut idx = 1;
        while idx < arr.len() {
            Self::create_node(root.clone(), arr[idx]);
            idx += 1;
        }

        return root;
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
