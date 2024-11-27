// Time taken: 20:07, 20:12 -> Acc

struct MyCircularQueue {
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
impl MyCircularQueue {
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

    fn en_queue(&mut self, value: i32) -> bool {
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

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.arr[self.head] = -1;
        self.length -= 1;
        if self.length == 0 {
            self.head = 0;
            self.tail = 0;
        } else {
            self.head = (self.head + 1) % self.size;
        }

        return true;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.arr[self.head];
    }

    fn rear(&self) -> i32 {
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
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
