// Time taken: 13:34, 13:48 -> Wrong, 13:51 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(mut arr) = self.map.get_mut(&key) {
            Self::binary_insert(&mut arr, value, timestamp);
        } else {
            self.map.insert(key, vec![(timestamp, value)]);
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(arr) = self.map.get(&key) {
            return Self::binary_search(&arr, timestamp);
        } else {
            return String::new();
        }
    }

    pub fn binary_search(arr: &Vec<(i32, String)>, timestamp: i32) -> String {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].0.cmp(&timestamp) {
                Ordering::Equal => {
                    return arr[mid].1.clone();
                }
                Ordering::Greater => {
                    right = mid;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        if left == arr.len() {
            return arr[left - 1].1.clone();
        } else {
            if arr[left].0 < timestamp {
                return arr[left].1.clone();
            } else {
                if left == 0 {
                    return String::new();
                } else {
                    return arr[left - 1].1.clone();
                }
            }
        }
    }

    pub fn binary_insert(arr: &mut Vec<(i32, String)>, value: String, timestamp: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].0.cmp(&timestamp) {
                Ordering::Equal => {
                    arr[mid] = (timestamp, value);
                    return;
                }
                Ordering::Greater => {
                    right = mid;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        arr.insert(left, (timestamp, value));
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
