// Time taken: 20:20, 20:28 -> Acc
struct MyHashMap {
    table: Vec<Option<Vec<(i32, i32)>>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        let prime = 7919;
        let table = vec![None; prime];
        Self {
            table: table,
            capacity: prime,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let position = key.abs() as usize % self.capacity;
        if let Some(list) = self.table[position].as_mut() {
            for (idx, (k1, v1)) in list.iter().enumerate() {
                if *k1 == key {
                    list[idx] = (*k1, value);
                    return;
                }
            }
            list.push((key, value));
        } else {
            self.table[position] = Some(vec![(key, value)]);
        }
    }

    fn get(&self, key: i32) -> i32 {
        let position = key.abs() as usize % self.capacity;
        if let Some(list) = self.table[position].as_ref() {
            for (k1, v1) in list.iter() {
                if *k1 == key {
                    return *v1;
                }
            }
            return -1;
        } else {
            return -1;
        }
    }

    fn remove(&mut self, key: i32) {
        let position = key.abs() as usize % self.capacity;
        if let Some(list) = self.table[position].as_mut() {
            for (idx, (k1, v1)) in list.iter().enumerate() {
                if *k1 == key {
                    list.remove(idx);
                    return;
                }
            }
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
