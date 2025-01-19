// Time taken: 23:06, 23:19 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Deref,
    rc::Rc,
};

struct Solution {}

impl Solution {}

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

struct WordDictionary {
    root: Option<Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        return Self {
            root: Some(Rc::new(RefCell::new(Node::new('.')))),
        };
    }

    fn add_word(&mut self, word: String) {
        let mut curr = self.root.clone();
        let word = word.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let a = 'a' as u8;

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

    fn search_rec(node: Option<Rc<RefCell<Node>>>, idx: usize, word: &Vec<char>) -> bool {
        // Base condition
        if node.is_none() {
            return false;
        }
        if idx >= word.len() {
            return node.unwrap().borrow().end;
        }

        let node = node.unwrap();
        let node = node.borrow();
        let mut result = false;

        if word[idx] == '.' {
            for idx2 in 0..26 {
                result = result | Self::search_rec(node.next[idx2].clone(), idx + 1, &word);
                if result {
                    return true;
                }
            }
            return result;
        } else {
            return Self::search_rec(
                node.next[(word[idx] as u8 - 'a' as u8) as usize].clone(),
                idx + 1,
                &word,
            );
        }
    }

    fn search(&self, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        return Self::search_rec(self.root.clone(), 0, &word);
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
