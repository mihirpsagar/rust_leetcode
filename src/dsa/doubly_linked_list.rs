use std::{cell::RefCell, rc::Rc};

struct Node {
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        return Self {
            val: val,
            prev: None,
            next: None,
        };
    }
}

struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::new(-1)));
        let tail = Rc::new(RefCell::new(Node::new(-1)));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        return Self {
            head: Some(head),
            tail: Some(tail),
            len: 0,
        };
    }

    pub fn push_back(&mut self, val: i32) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        let prev_node = self.tail.clone().unwrap().borrow().prev.clone().unwrap();
        let next_node = self.tail.clone().unwrap();

        prev_node.borrow_mut().next = Some(node.clone());
        next_node.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().prev = Some(prev_node.clone());
        node.borrow_mut().next = Some(next_node.clone());

        self.len += 1;
    }

    pub fn push_front(&mut self, val: i32) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        let prev_node = self.head.clone().unwrap();
        let next_node = self.head.clone().unwrap().borrow().next.clone().unwrap();

        prev_node.borrow_mut().next = Some(node.clone());
        next_node.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().prev = Some(prev_node.clone());
        node.borrow_mut().next = Some(next_node.clone());

        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        let node = self.tail.clone().unwrap().borrow().prev.clone().unwrap();
        let prev_node = node.borrow().prev.clone().unwrap();
        let next_node = self.tail.clone().unwrap();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node.clone());
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;

        self.len -= 1;
        return Some(node.borrow().val);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        let node = self.head.clone().unwrap().borrow().next.clone().unwrap();
        let prev_node = self.head.clone().unwrap();
        let next_node = node.borrow().next.clone().unwrap();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node.clone());
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;

        self.len -= 1;
        return Some(node.borrow().val);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_doubly_linked_list() {
        let mut list = DoublyLinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));

        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));

        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
    }
}
