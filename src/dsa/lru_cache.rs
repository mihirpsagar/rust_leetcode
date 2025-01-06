use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(key: i32, val: i32) -> Self {
        return Self {
            key: key,
            val: val,
            prev: None,
            next: None,
        };
    }
}

struct LRUCache {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
    capacity: usize,
    map: HashMap<i32, Option<Rc<RefCell<Node>>>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        let head = Rc::new(RefCell::new(Node::new(-1, -1)));
        let tail = Rc::new(RefCell::new(Node::new(-1, -1)));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        return Self {
            head: Some(head),
            tail: Some(tail),
            len: 0,
            capacity: capacity,
            map: HashMap::new(),
        };
    }

    fn remove_node(&self, node: Option<Rc<RefCell<Node>>>) {
        let node = node.unwrap();
        let prev_node = node.borrow().prev.clone().unwrap();
        let next_node = node.borrow().next.clone().unwrap();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node.clone());

        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }

    fn push_back_node(&self, node: Option<Rc<RefCell<Node>>>) {
        let prev_node = self.tail.clone().unwrap().borrow().prev.clone().unwrap();
        let next_node = self.tail.clone().unwrap();

        prev_node.borrow_mut().next = node.clone();
        next_node.borrow_mut().prev = node.clone();

        node.clone().unwrap().borrow_mut().prev = Some(prev_node.clone());
        node.clone().unwrap().borrow_mut().next = Some(next_node.clone());
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(node) = self.map.get(&key) {
            self.remove_node(node.clone());
            self.push_back_node(node.clone());
            return Some(node.clone().unwrap().borrow().val);
        } else {
            return None;
        }
    }

    pub fn set(&mut self, key: i32, val: i32) {
        if let Some(node) = self.map.get(&key) {
            self.remove_node(node.clone());
            self.push_back_node(node.clone());
            node.clone().unwrap().borrow_mut().val = val;
        } else {
            let node = Some(Rc::new(RefCell::new(Node::new(key, val))));
            self.map.insert(key, node.clone());
            if self.len == self.capacity {
                let remove_node = self.head.clone().unwrap().borrow().next.clone();
                let remove_key = remove_node.clone().unwrap().borrow().key;
                self.map.remove_entry(&remove_key);
                self.remove_node(remove_node);
            } else {
                self.len += 1;
            }
            self.push_back_node(node.clone());
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(3);

        lru_cache.set(1, 100);
        assert_eq!(lru_cache.get(1), Some(100));
        assert_eq!(lru_cache.get(2), None);

        lru_cache.set(2, 200);
        lru_cache.set(3, 300);
        assert_eq!(lru_cache.get(2), Some(200));
        assert_eq!(lru_cache.get(1), Some(100));
        assert_eq!(lru_cache.get(3), Some(300));

        lru_cache.set(4, 400);
        assert_eq!(lru_cache.get(2), None);
    }
}
