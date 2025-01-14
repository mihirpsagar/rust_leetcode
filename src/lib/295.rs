// Time taken: 20:15, 20:45 -> Wrong, 20:51 -> Acc, 21:10 -> Optimized

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {}

struct MedianFinder {
    left_max_heap: BinaryHeap<i32>,
    right_min_heap: BinaryHeap<Reverse<i32>>,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        return Self {
            left_max_heap: BinaryHeap::new(),
            right_min_heap: BinaryHeap::new(),
            len: 0,
        };
    }

    fn add_num(&mut self, num: i32) {
        self.right_min_heap.push(Reverse(num));

        if (self.right_min_heap.len() - self.left_max_heap.len()) > 1 {
            let val = self.right_min_heap.pop().unwrap().0;
            self.left_max_heap.push(val);
        } else if !self.left_max_heap.is_empty() {
            let left_val = *self.left_max_heap.peek().unwrap();
            let right_val = self.right_min_heap.peek().unwrap().0;
            if left_val > right_val {
                self.left_max_heap.pop();
                self.right_min_heap.pop();
                self.left_max_heap.push(right_val);
                self.right_min_heap.push(Reverse(left_val));
            }
        }

        self.len += 1;
    }

    // fn add_num(&mut self, num: i32) {
    //     let right_len = self.right_min_heap.len();
    //     let left_len = self.left_max_heap.len();

    //     let left_node = self.left_max_heap.peek();
    //     let right_node = self.right_min_heap.peek();

    //     match (left_node, right_node) {
    //         (None, None) => {
    //             self.right_min_heap.push(Reverse(num));
    //         }
    //         (None, Some(right_val)) => {
    //             if num < right_val.0 {
    //                 self.left_max_heap.push(num);
    //             } else {
    //                 self.left_max_heap.push(right_val.0);
    //                 self.right_min_heap.pop();
    //                 self.right_min_heap.push(Reverse(num));
    //             }
    //         }
    //         (Some(left_val), Some(right_val)) => {
    //             // Step 1: Check which direction the num should be inserted
    //             let mut direction = 'L';
    //             if num > right_val.0 {
    //                 direction = 'R';
    //             }
    //             if num >= *left_val || num == right_val.0 {
    //                 if self.len % 2 == 0 {
    //                     direction = 'R';
    //                 }
    //             }

    //             // Step 2: If direction is left, push left and swap if left_len == right_len.
    //             // If direction is right, push right and swap if left_len < right_len.
    //             if direction == 'L' {
    //                 if left_len == right_len {
    //                     self.right_min_heap.push(Reverse(*left_val));
    //                     self.left_max_heap.pop();
    //                 }
    //                 self.left_max_heap.push(num);
    //             } else {
    //                 if left_len < right_len {
    //                     self.left_max_heap.push(right_val.0);
    //                     self.right_min_heap.pop();
    //                 }
    //                 self.right_min_heap.push(Reverse(num));
    //             }
    //         }
    //         _ => {
    //             unreachable!();
    //         }
    //     }

    //     self.len += 1;
    // }

    fn find_median(&self) -> f64 {
        let right_val = self.right_min_heap.peek().unwrap().0;
        if self.len % 2 == 0 {
            let left_val = *self.left_max_heap.peek().unwrap();
            return (left_val as f64 + right_val as f64) / 2.0;
        } else {
            return right_val as f64;
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
