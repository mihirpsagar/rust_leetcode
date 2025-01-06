struct RingBuffer {
    arr: Vec<i32>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        let arr = vec![0; 5];
        return Self {
            arr: arr,
            head: 0,
            tail: 0,
            len: 0,
            capacity: 5,
        };
    }

    fn increase_capacity(&mut self) {
        let mut new_arr = vec![0; self.capacity * 2];
        let mut idx1 = 0;
        let mut idx2 = self.head;
        for _ in 0..self.len {
            new_arr[idx1] = self.arr[idx2];
            idx1 += 1;
            idx2 = (idx2 + 1) % self.capacity;
        }
        self.arr = new_arr;
        self.head = 0;
        self.tail = self.len;
        self.capacity *= 2;
    }

    pub fn push_back(&mut self, val: i32) {
        if self.len == self.capacity {
            self.increase_capacity();
        }
        self.arr[self.tail] = val;
        self.tail = (self.tail + 1) % self.capacity;
        self.len += 1;
    }

    pub fn push_front(&mut self, val: i32) {
        if self.len == self.capacity {
            self.increase_capacity();
        }

        if self.head == 0 {
            self.head = self.capacity - 1;
        } else {
            self.head -= 1;
        }

        self.arr[self.head] = val;
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        if self.tail == 0 {
            self.tail = self.capacity - 1;
        } else {
            self.tail -= 1;
        }

        self.len -= 1;
        return Some(self.arr[self.tail]);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        let result = Some(self.arr[self.head]);
        self.head = (self.head + 1) % self.capacity;

        self.len -= 1;
        return result;
    }
}

#[cfg(test)]
pub mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    pub fn test_ring_buffer() {
        let mut ring_buffer = RingBuffer::new();

        for i in 1..=5 {
            ring_buffer.push_back(i);
        }

        for i in 6..=10 {
            ring_buffer.push_front(i);
        }

        assert_eq!(ring_buffer.pop_back(), Some(5));
        assert_eq!(ring_buffer.pop_front(), Some(10));
        assert_eq!(ring_buffer.pop_back(), Some(4));
        assert_eq!(ring_buffer.pop_front(), Some(9));
        assert_eq!(ring_buffer.pop_back(), Some(3));
        assert_eq!(ring_buffer.pop_front(), Some(8));
        assert_eq!(ring_buffer.pop_back(), Some(2));
        assert_eq!(ring_buffer.pop_front(), Some(7));
        assert_eq!(ring_buffer.pop_back(), Some(1));
        assert_eq!(ring_buffer.pop_front(), Some(6));
    }
}
