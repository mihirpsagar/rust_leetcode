use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use rand::Rng;

struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        return Self {
            val: val,
            left: None,
            right: None,
        };
    }
}

pub fn create_tree(arr: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
    if arr.len() == 0 {
        return None;
    }

    let root = Rc::new(RefCell::new(Node::new(arr[0])));
    let mut queue = VecDeque::new();
    queue.push_back((root.clone(), 0));

    while let Some(val) = queue.pop_front() {
        let l = (2 * val.1) + 1;
        let r = (2 * val.1) + 2;
        let node = val.0;

        if l < arr.len() {
            let left_node = Rc::new(RefCell::new(Node::new(arr[l])));
            node.borrow_mut().left = Some(left_node.clone());
            queue.push_back((left_node, l));
        }

        if r < arr.len() {
            let right_node = Rc::new(RefCell::new(Node::new(arr[r])));
            node.borrow_mut().right = Some(right_node.clone());
            queue.push_back((right_node, r));
        }
    }

    return Some(root);
}

pub fn dfs_recursive(node: Option<Rc<RefCell<Node>>>, target: i32) -> bool {
    if node.is_none() {
        return false;
    }

    let node = node.unwrap();

    if node.borrow().val == target {
        return true;
    }

    return dfs_recursive(node.borrow().left.clone(), target)
        || dfs_recursive(node.borrow().right.clone(), target);
}

pub fn dfs_iterative(root: Option<Rc<RefCell<Node>>>, target: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let mut stack = Vec::new();
    stack.push(root);

    while let Some(node) = stack.pop() {
        if node.is_none() {
            continue;
        }

        let node = node.unwrap();
        if node.borrow().val == target {
            return true;
        }

        if node.borrow().right.is_some() {
            stack.push(node.borrow().right.clone());
        }

        if node.borrow().left.is_some() {
            stack.push(node.borrow().left.clone());
        }
    }

    return false;
}

pub fn create_random_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..=100) as usize;
    let mut arr = Vec::new();
    let excluded_val = 42;

    for _ in 0..len {
        let val = rng.gen_range(i32::MIN..=i32::MAX);
        if val != excluded_val {
            arr.push(val);
        } else {
            arr.push(val + 1);
        }
    }

    return arr;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_dfs() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let arr = create_random_array();
            let len = arr.len();
            let root = create_tree(arr.clone());

            assert_eq!(dfs_recursive(root.clone(), 42), false);
            assert_eq!(dfs_iterative(root.clone(), 42), false);

            for _ in 0..10 {
                let idx = rng.gen_range(0..len);
                assert_eq!(dfs_recursive(root.clone(), arr[idx]), true);
                assert_eq!(dfs_iterative(root.clone(), arr[idx]), true);
            }
        }
    }
}
