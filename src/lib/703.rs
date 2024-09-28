use std::{cmp::Reverse, collections::BinaryHeap};

// Time taken: 14:08, 14:17 -> Acc
struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::with_capacity(k as usize);
        let k = k as usize;

        for num in nums {
            if min_heap.len() < k {
                min_heap.push(Reverse(num));
            } else {
                if min_heap.peek().unwrap().0 < num {
                    min_heap.pop();
                    min_heap.push(Reverse(num));
                }
            }
        }

        return Self {
            min_heap: min_heap,
            capacity: k,
        };
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.capacity {
            self.min_heap.push(Reverse(val));
        } else if let Some(min_val) = self.min_heap.peek() {
            if min_val.0 < val {
                self.min_heap.pop();
                self.min_heap.push(Reverse(val));
            }
        }
        return self.min_heap.peek().unwrap().0;
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
