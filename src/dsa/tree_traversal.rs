use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Debug)]
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

pub fn pre_order_traversal_iterative(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    stack.push(root);

    while let Some(node) = stack.pop() {
        if node.is_none() {
            continue;
        }

        let node = node.unwrap();
        result.push(node.borrow().val);
        stack.push(node.borrow().right.clone());
        stack.push(node.borrow().left.clone());
    }

    return result;
}

pub fn in_order_traversal_iterative(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;

    while curr.is_some() || !stack.is_empty() {
        while curr.is_some() {
            stack.push(curr.clone());
            curr = curr.unwrap().borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            if node.is_none() {
                continue;
            }

            let node = node.unwrap();
            result.push(node.borrow().val);
            curr = node.borrow().right.clone();
        }
    }

    return result;
}

pub fn post_order_traversal_iterative(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    stack.push(root);

    while let Some(node) = stack.pop() {
        if node.is_none() {
            continue;
        }

        let node = node.unwrap();
        result.push(node.borrow().val);
        stack.push(node.borrow().left.clone());
        stack.push(node.borrow().right.clone());
    }

    result.reverse();
    return result;
}

pub fn pre_order_traversal_recursive(node: Option<Rc<RefCell<Node>>>, mut arr: &mut Vec<i32>) {
    if let Some(node) = node {
        arr.push(node.borrow().val);
        pre_order_traversal_recursive(node.borrow().left.clone(), &mut arr);
        pre_order_traversal_recursive(node.borrow().right.clone(), &mut arr);
    }
}

pub fn in_order_traversal_recursive(node: Option<Rc<RefCell<Node>>>, mut arr: &mut Vec<i32>) {
    if let Some(node) = node {
        in_order_traversal_recursive(node.borrow().left.clone(), &mut arr);
        arr.push(node.borrow().val);
        in_order_traversal_recursive(node.borrow().right.clone(), &mut arr);
    }
}

pub fn post_order_traversal_recursive(node: Option<Rc<RefCell<Node>>>, mut arr: &mut Vec<i32>) {
    if let Some(node) = node {
        post_order_traversal_recursive(node.borrow().left.clone(), &mut arr);
        post_order_traversal_recursive(node.borrow().right.clone(), &mut arr);
        arr.push(node.borrow().val);
    }
}

// pub fn pre_order_morris_traversal(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
//     let mut result = Vec::new();
//     let mut curr = root;
//     let mut count = 0;

//     while curr.clone().is_some() {
//         println!("Start: {}", curr.clone().unwrap().borrow().val);
//         if curr.clone().unwrap().borrow().left.is_none() {
//             println!("No left");
//             result.push(curr.clone().unwrap().borrow().val);
//             curr = curr.clone().unwrap().borrow().right.clone();
//         } else {
//             let mut right_most = curr.clone().unwrap().borrow().left.clone();
//             println!("Right found: {}", right_most.clone().unwrap().borrow().val);
//             // println!("{:?}", right_most);
//             while right_most.clone().unwrap().borrow().right.is_some()
//                 && right_most.clone().unwrap().borrow().right != curr
//             {
//                 println!("Right Iter: {}", right_most.clone().unwrap().borrow().val);
//                 right_most = right_most.clone().unwrap().borrow().right.clone();
//             }

//             if right_most.clone().unwrap().borrow().right.is_none() {
//                 println!(
//                     "Pointing: {} to {}",
//                     right_most.clone().unwrap().borrow().val,
//                     curr.clone().unwrap().borrow().val
//                 );
//                 right_most.unwrap().borrow_mut().right = curr.clone();
//                 result.push(curr.clone().unwrap().borrow().val);
//                 curr = curr.clone().unwrap().borrow().left.clone();
//             } else {
//                 println!(
//                     "Removing thread: {}",
//                     right_most.clone().unwrap().borrow().val
//                 );
//                 right_most.clone().unwrap().borrow_mut().right = None;
//                 curr = curr.clone().unwrap().borrow().right.clone();
//             }
//         }
//     }

//     return result;
// }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_pre_order_traversal() {
        let root = create_tree(vec![1, 2, 3, 4, 5, 6, 7]);
        let result = vec![1, 2, 4, 5, 3, 6, 7];
        assert_eq!(pre_order_traversal_iterative(root.clone()), result);

        // let mut node1 = Node::new(1);
        // let mut node2 = Node::new(2);
        // let node3 = Rc::new(RefCell::new(Node::new(3)));
        // node1.right = Some(node3.clone());
        // node2.left = Some(node3.clone());
        // println!("{}", node1.right == node2.left);

        // assert_eq!(pre_order_morris_traversal(root.clone()), result);

        let mut arr = Vec::new();
        pre_order_traversal_recursive(root, &mut arr);
        assert_eq!(arr, result);
    }

    #[test]
    pub fn test_in_order_traversal() {
        let root = create_tree(vec![1, 2, 3, 4, 5, 6, 7]);
        let result = vec![4, 2, 5, 1, 6, 3, 7];
        assert_eq!(in_order_traversal_iterative(root.clone()), result);
        let mut arr = Vec::new();
        in_order_traversal_recursive(root, &mut arr);
        assert_eq!(arr, result);
    }

    #[test]
    pub fn test_post_order_traversal() {
        let root = create_tree(vec![1, 2, 3, 4, 5, 6, 7]);
        let result = vec![4, 5, 2, 6, 7, 3, 1];
        assert_eq!(post_order_traversal_iterative(root.clone()), result);
        let mut arr = Vec::new();
        post_order_traversal_recursive(root, &mut arr);
        assert_eq!(arr, result);
    }
}
