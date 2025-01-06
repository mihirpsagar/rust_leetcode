use rand::Rng;
use std::{cell::RefCell, rc::Rc};

struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        return Self {
            val: val,
            next: None,
        };
    }
}

pub fn create_linked_list(arr: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
    if arr.len() == 0 {
        return None;
    }

    let head = Rc::new(RefCell::new(Node::new(arr[0])));
    let mut curr = head.clone();

    let mut idx = 1;
    while idx < arr.len() {
        let next_node = Rc::new(RefCell::new(Node::new(arr[idx])));
        curr.borrow_mut().next = Some(next_node.clone());
        curr = next_node;
        idx += 1;
    }

    return Some(head);
}

pub fn iterate_linked_list(head: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head.clone();
    while curr.is_some() {
        result.push(curr.clone().unwrap().borrow().val);
        curr = curr.unwrap().borrow().next.clone();
    }

    return result;
}

pub fn reverse_linked_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut prev = None;
    let mut curr = head.clone();
    while curr.is_some() {
        let next_node = curr.clone().unwrap().borrow().next.clone();
        curr.clone().unwrap().borrow_mut().next = prev;
        prev = curr;
        curr = next_node;
    }

    return prev;
}

pub fn create_random_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..=10) as usize;
    let mut arr = Vec::new();
    for _ in 0..len {
        arr.push(rng.gen_range(i32::MIN..=i32::MAX));
    }

    return arr;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_reverse_linked_list() {
        for _ in 0..10 {
            let mut arr = create_random_array();
            let head = create_linked_list(arr.clone());
            let new_head = reverse_linked_list(head.clone());
            arr.reverse();
            assert_eq!(iterate_linked_list(new_head), arr);
            assert_eq!(iterate_linked_list(head).len(), 1);
        }
    }
}
