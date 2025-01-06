// Time taken: 22:22, 22:41 -> Acc

use std::collections::{HashMap, VecDeque};

struct LRUCache {
    queue: VecDeque<i32>,
    map: HashMap<i32, i32>,
    size: i32,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        return Self {
            queue: VecDeque::with_capacity(capacity as usize),
            map: HashMap::with_capacity(capacity as usize),
            size: 0,
            capacity: capacity,
        };
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(val) = self.map.get(&key) {
            let mut queue_index = 0;
            for queue_key in self.queue.iter() {
                if *queue_key == key {
                    break;
                }
                queue_index += 1;
            }
            self.queue.remove(queue_index);
            self.queue.push_front(key);
            return *val;
        } else {
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let mut queue_index = 0;
            for queue_key in self.queue.iter() {
                if *queue_key == key {
                    break;
                }
                queue_index += 1;
            }
            self.queue.remove(queue_index);
            self.queue.push_front(key);
            self.map.insert(key, value);
        } else {
            if self.size < self.capacity {
                self.map.insert(key, value);
                self.queue.push_front(key);
                self.size += 1;
            } else {
                if let Some(lru_key) = self.queue.pop_back() {
                    self.map.remove(&lru_key);
                }
                self.map.insert(key, value);
                self.queue.push_front(key);
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
