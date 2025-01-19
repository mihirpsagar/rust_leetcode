// Time taken: 23:29, 23:54 -> TLE, 00:16 -> TLE, 00:22, 00:33 -> Acc, 01:08 -> Optimized

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Deref,
    rc::Rc,
};

struct Solution {}

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

impl Trie {
    pub fn new() -> Self {
        return Self {
            root: Some(Rc::new(RefCell::new(Node::new('.')))),
        };
    }

    pub fn add(&mut self, word: Vec<char>) {
        let mut curr = self.root.clone();
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

    pub fn search(&self, word: Vec<char>) -> bool {
        let mut curr = self.root.clone();
        let mut idx = 0;
        let a = 'a' as u8;

        while idx < word.len() {
            if let Some(inner) = curr {
                curr = inner.borrow().next[(word[idx] as u8 - a) as usize].clone();
            } else {
                return false;
            }
            idx += 1;
        }

        return curr.is_some();
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            let word = word.chars().collect::<Vec<char>>();
            trie.add(word);
        }

        let mut result_set = HashSet::new();
        let mut idx1 = 0;
        while idx1 < board.len() {
            let mut idx2 = 0;
            while idx2 < board[0].len() {
                Self::dfs(
                    idx1,
                    idx2,
                    trie.root.clone().unwrap().borrow().next
                        [(board[idx1][idx2] as u8 - 'a' as u8) as usize]
                        .clone(),
                    &mut Vec::new(),
                    &mut board,
                    &mut result_set,
                );
                idx2 += 1;
            }
            idx1 += 1;
        }

        let mut result = Vec::new();
        for word in result_set {
            result.push(word);
        }
        return result;
    }

    fn dfs(
        row: usize,
        col: usize,
        node: Option<Rc<RefCell<Node>>>,
        mut curr: &mut Vec<char>,
        mut board: &mut Vec<Vec<char>>,
        mut result_set: &mut HashSet<String>,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node = node.borrow();

        let ch = board[row][col];
        curr.push(ch);
        board[row][col] = '#';

        // Right
        if col + 1 < board[0].len() && board[row][col + 1] != '#' {
            Self::dfs(
                row,
                col + 1,
                node.next[(board[row][col + 1] as u8 - 'a' as u8) as usize].clone(),
                &mut curr,
                &mut board,
                &mut result_set,
            );
        }

        // Bottom
        if row + 1 < board.len() && board[row + 1][col] != '#' {
            Self::dfs(
                row + 1,
                col,
                node.next[(board[row + 1][col] as u8 - 'a' as u8) as usize].clone(),
                &mut curr,
                &mut board,
                &mut result_set,
            );
        }

        // Left
        if col != 0 && board[row][col - 1] != '#' {
            Self::dfs(
                row,
                col - 1,
                node.next[(board[row][col - 1] as u8 - 'a' as u8) as usize].clone(),
                &mut curr,
                &mut board,
                &mut result_set,
            );
        }

        // Top
        if row != 0 && board[row - 1][col] != '#' {
            Self::dfs(
                row - 1,
                col,
                node.next[(board[row - 1][col] as u8 - 'a' as u8) as usize].clone(),
                &mut curr,
                &mut board,
                &mut result_set,
            );
        }

        if node.end {
            result_set.insert(curr.iter().collect());
        }

        curr.pop();
        board[row][col] = ch;
    }
}

// impl Solution {
//     pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
//         let trie = Trie::new();

//         let mut idx1 = 0;
//         while idx1 < board.len() {
//             let mut idx2 = 0;
//             while idx2 < board[0].len() {
//                 Self::dfs(idx1, idx2, 0, &mut board, trie.root.clone());
//                 idx2 += 1;
//             }
//             idx1 += 1;
//         }

//         // let mut curr = Vec::new();
//         // let mut insert_set = HashSet::new();
//         // let mut idx1 = 0;
//         // while idx1 < board.len() {
//         //     let mut idx2 = 0;
//         //     while idx2 < board[0].len() {
//         //         Self::dfs(
//         //             idx1,
//         //             idx2,
//         //             &mut curr,
//         //             &mut board,
//         //             &mut trie,
//         //             &mut insert_set,
//         //         );
//         //         idx2 += 1;
//         //     }
//         //     idx1 += 1;
//         // }

//         let mut result = Vec::new();
//         for word in words {
//             let word_arr = word.chars().collect::<Vec<char>>();
//             if trie.search(word_arr) {
//                 result.push(word.clone());
//             }
//         }

//         return result;
//     }

//     fn dfs(
//         row: usize,
//         col: usize,
//         len: usize,
//         mut board: &mut Vec<Vec<char>>,
//         node: Option<Rc<RefCell<Node>>>,
//     ) {
//         // Base condition
//         if row >= board.len() || col >= board[0].len() {
//             return;
//         }
//         if board[row][col] == '#' {
//             return;
//         }
//         if len >= 10 {
//             return;
//         }

//         let ch = board[row][col];
//         board[row][col] = '#';
//         let node = node.unwrap();
//         let mut node = node.borrow_mut();
//         if node.next[(ch as u8 - 'a' as u8) as usize].is_none() {
//             node.next[(ch as u8 - 'a' as u8) as usize] = Some(Rc::new(RefCell::new(Node::new(ch))));
//         }

//         // Right
//         if col + 1 < board[0].len() {
//             Self::dfs(
//                 row,
//                 col + 1,
//                 len + 1,
//                 &mut board,
//                 node.next[(ch as u8 - 'a' as u8) as usize].clone(),
//             );
//         }

//         // Bottom
//         if row + 1 < board.len() {
//             Self::dfs(
//                 row + 1,
//                 col,
//                 len + 1,
//                 &mut board,
//                 node.next[(ch as u8 - 'a' as u8) as usize].clone(),
//             );
//         }

//         // Left
//         if col != 0 {
//             Self::dfs(
//                 row,
//                 col - 1,
//                 len + 1,
//                 &mut board,
//                 node.next[(ch as u8 - 'a' as u8) as usize].clone(),
//             );
//         }

//         // Top
//         if row != 0 {
//             Self::dfs(
//                 row - 1,
//                 col,
//                 len + 1,
//                 &mut board,
//                 node.next[(ch as u8 - 'a' as u8) as usize].clone(),
//             );
//         }

//         board[row][col] = ch;
//     }

//     // fn dfs(
//     //     row: usize,
//     //     col: usize,
//     //     mut curr: &mut Vec<char>,
//     //     mut board: &mut Vec<Vec<char>>,
//     //     mut trie: &mut Trie,
//     //     mut insert_set: &mut HashSet<Vec<char>>,
//     // ) {
//     //     // Base condition
//     //     if row >= board.len() || col >= board[0].len() {
//     //         return;
//     //     }
//     //     // If already visited, return.
//     //     if board[row][col] == '#' {
//     //         return;
//     //     }

//     //     let ch = board[row][col];
//     //     curr.push(ch);
//     //     board[row][col] = '#';

//     //     // Right
//     //     if col + 1 < board[0].len() {
//     //         Self::dfs(
//     //             row,
//     //             col + 1,
//     //             &mut curr,
//     //             &mut board,
//     //             &mut trie,
//     //             &mut insert_set,
//     //         );
//     //     }

//     //     // Bottom
//     //     if row + 1 < board.len() {
//     //         Self::dfs(
//     //             row + 1,
//     //             col,
//     //             &mut curr,
//     //             &mut board,
//     //             &mut trie,
//     //             &mut insert_set,
//     //         );
//     //     }

//     //     // Left
//     //     if col != 0 {
//     //         Self::dfs(
//     //             row,
//     //             col - 1,
//     //             &mut curr,
//     //             &mut board,
//     //             &mut trie,
//     //             &mut insert_set,
//     //         );
//     //     }

//     //     // Top
//     //     if row != 0 {
//     //         Self::dfs(
//     //             row - 1,
//     //             col,
//     //             &mut curr,
//     //             &mut board,
//     //             &mut trie,
//     //             &mut insert_set,
//     //         );
//     //     }

//     //     // Add to trie if not already added
//     //     if !insert_set.contains(curr) {
//     //         trie.add(curr.clone());
//     //         insert_set.insert(curr.clone());
//     //     }

//     //     curr.pop();
//     //     board[row][col] = ch;
//     // }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
