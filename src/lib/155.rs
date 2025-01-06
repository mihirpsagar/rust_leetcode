// Time taken: 21:03, 21:12 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct MinStack {
    arr: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        return Self { arr: Vec::new() };
    }

    fn push(&mut self, val: i32) {
        if self.arr.is_empty() {
            self.arr.push((val, val));
            return;
        }

        let min = std::cmp::min(self.arr[self.arr.len() - 1].1, val);
        self.arr.push((val, min));
    }

    fn pop(&mut self) {
        self.arr.pop();
    }

    fn top(&self) -> i32 {
        return self.arr[self.arr.len() - 1].0;
    }

    fn get_min(&self) -> i32 {
        return self.arr[self.arr.len() - 1].1;
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
