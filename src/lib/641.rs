// Time taken: 20:16, 20:26 -> Wrong, 20:27 -> Acc

struct MyCircularDeque {
    arr: Vec<i32>,
    length: usize,
    size: usize,
    head: usize,
    tail: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let k = k as usize;
        return Self {
            arr: vec![-1; k],
            length: 0,
            size: k,
            head: 0,
            tail: 0,
        };
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.length != 0 {
            if self.head == 0 {
                self.head = self.size - 1;
            } else {
                self.head -= 1;
            }
        }
        self.arr[self.head] = value;
        self.length += 1;

        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.length != 0 {
            self.tail = (self.tail + 1) % self.size;
        }
        self.arr[self.tail] = value;
        self.length += 1;

        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.arr[self.head] = -1;
        self.length -= 1;
        if self.length != 0 {
            self.head = (self.head + 1) % self.size;
        }

        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.arr[self.tail] = -1;
        self.length -= 1;
        if self.length != 0 {
            if self.tail == 0 {
                self.tail = self.size - 1;
            } else {
                self.tail -= 1;
            }
        }

        return true;
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.arr[self.head];
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.arr[self.tail];
    }

    fn is_empty(&self) -> bool {
        return self.length == 0;
    }

    fn is_full(&self) -> bool {
        return self.length == self.size;
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
