use std::collections::VecDeque;

// Time taken: 01:47, 01:56 -> Acc
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        if self.queue.is_empty() {
            self.queue.push_back(t);
        } else {
            loop {
                if let Some(&node) = self.queue.front() {
                    if t - node > 3000 {
                        self.queue.pop_front();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.queue.push_back(t);
        }

        return self.queue.len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
