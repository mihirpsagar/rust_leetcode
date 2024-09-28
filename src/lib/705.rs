// Time taken: 19:55, 20:17 -> Acc
struct MyHashSet {
    table: Vec<Option<Vec<i32>>>,
    capacity: usize,
    prime: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        let prime = 7919;
        // let capacity = 10_000;
        let table = vec![None; prime];
        Self {
            table: table,
            capacity: prime,
            prime: prime,
        }
    }

    fn add(&mut self, key: i32) {
        let position = key.abs() as usize % self.prime;
        if self.table[position].is_none() {
            self.table[position] = Some(vec![key]);
        } else {
            let list = self.table[position].as_mut().unwrap();
            for &num in list.iter() {
                if num == key {
                    return;
                }
            }
            list.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let position = key.abs() as usize % self.prime;
        if self.table[position].is_none() {
            return;
        } else {
            let list = self.table[position].as_mut().unwrap();
            let mut idx = 0;
            while idx < list.len() {
                if list[idx] == key {
                    list.remove(idx);
                    break;
                }
                idx += 1;
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        let position = key.abs() as usize % self.prime;
        if self.table[position].is_none() {
            return false;
        } else {
            let list = self.table[position].as_ref().unwrap();
            for &num in list {
                if num == key {
                    return true;
                }
            }
            return false;
        }
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
