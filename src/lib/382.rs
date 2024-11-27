// Time taken: 13:22, 13:38 -> Acc

use ::rand::prelude::*;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {
    head: Option<Box<ListNode>>,
    size: u32,
    found_size: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        return Self {
            head: head,
            size: 100,
            found_size: false,
        };
    }

    fn get_random(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut curr = self.head.as_ref();
        let num = rng.gen_range(0..self.size);
        let mut count = 0;
        let mut result = -1;

        while curr.is_some() {
            result = curr.unwrap().val;
            if num == count {
                break;
            } else {
                curr = curr.unwrap().next.as_ref();
            }
            count += 1;
        }

        if !self.found_size {
            if curr.is_none() {
                self.found_size = true;
                self.size = count;
            } else {
                self.size += 100;
            }
        }

        return result;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
