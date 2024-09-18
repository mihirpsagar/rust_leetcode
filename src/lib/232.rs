// Time taken: 14:19, 14:27 -> Wrong, 14:32 -> Acc
struct Solution {}

impl Solution {}

struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack1.is_empty() {
            self.stack2.push(x);
        } else {
            self.stack1.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack1.is_empty() {
            while let Some(val) = self.stack2.pop() {
                self.stack1.push(val);
            }
            let result = self.stack1.pop().unwrap();
            while let Some(val) = self.stack1.pop() {
                self.stack2.push(val);
            }
            return result;
        } else {
            while let Some(val) = self.stack1.pop() {
                self.stack2.push(val);
            }
            let result = self.stack2.pop().unwrap();
            while let Some(val) = self.stack2.pop() {
                self.stack1.push(val);
            }
            return result;
        }
    }

    fn peek(&self) -> i32 {
        if self.stack1.is_empty() {
            return *self.stack2.get(0).unwrap();
        } else {
            return *self.stack1.get(0).unwrap();
        }
    }

    fn empty(&self) -> bool {
        return self.stack1.is_empty() && self.stack2.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
