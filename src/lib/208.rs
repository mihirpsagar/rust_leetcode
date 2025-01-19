// Time taken: 22:23, 22:56 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Deref,
    rc::Rc,
};

struct Node {
    val: char,
    next: Vec<Option<Rc<RefCell<Node>>>>,
    end: bool,
}

impl Node {
    pub fn new(val: char) -> Self {
        return Self {
            val: val,
            next: vec![None; 26],
            end: false,
        };
    }
}

struct Trie {
    root: Option<Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        return Self {
            root: Some(Rc::new(RefCell::new(Node::new('.')))),
        };
    }

    fn insert(&mut self, word: String) {
        let word = word.chars().collect::<Vec<char>>();
        let a = 'a' as u8;
        let mut idx = 0;
        let mut curr = self.root.clone();
        while idx < word.len() {
            if let Some(inner) = curr {
                let mut inner = inner.borrow_mut();
                if inner.next[(word[idx] as u8 - a) as usize].is_some() {
                    curr = inner.next[(word[idx] as u8 - a) as usize].clone();
                } else {
                    let new_node = Some(Rc::new(RefCell::new(Node::new(word[idx]))));
                    inner.next[(word[idx] as u8 - a) as usize] = new_node.clone();
                    curr = new_node;
                }
            }
            idx += 1;
        }

        curr.unwrap().borrow_mut().end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = self.root.clone();
        let word = word.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let a = 'a' as u8;

        while idx < word.len() {
            if let Some(inner) = curr {
                let inner = inner.borrow();
                curr = inner.next[(word[idx] as u8 - a) as usize].clone();
            } else {
                return false;
            }
            idx += 1;
        }

        if let Some(inner) = curr {
            return inner.borrow().end;
        } else {
            return false;
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self.root.clone();
        let prefix = prefix.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let a = 'a' as u8;

        while idx < prefix.len() {
            if let Some(inner) = curr {
                let inner = inner.borrow();
                curr = inner.next[(prefix[idx] as u8 - a) as usize].clone();
            } else {
                return false;
            }
            idx += 1;
        }

        return curr.is_some();
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
