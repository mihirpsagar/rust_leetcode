// Time taken: 23:48, 23:59 -> Acc

use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        for _ in 1..self.queue.len() {
            let val = self.queue.pop_front().unwrap();
            self.queue.push_back(val);
        }
        return self.queue.pop_front().unwrap();
    }

    fn top(&self) -> i32 {
        return *self.queue.back().unwrap();
    }

    fn empty(&self) -> bool {
        return self.queue.is_empty();
    }
}

struct Solution {}

impl Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
