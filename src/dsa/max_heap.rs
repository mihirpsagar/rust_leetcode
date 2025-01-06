struct MaxHeap {
    arr: Vec<i32>,
    len: usize,
}

impl MaxHeap {
    pub fn new() -> Self {
        return Self {
            arr: Vec::new(),
            len: 0,
        };
    }

    fn percolate_up(&mut self, node: usize) {
        if node == 0 {
            return;
        }

        let parent = (node - 1) / 2;

        if self.arr[parent] < self.arr[node] {
            let tmp = self.arr[parent];
            self.arr[parent] = self.arr[node];
            self.arr[node] = tmp;
            self.percolate_up(parent);
        }
    }

    fn heapify(&mut self, node: usize) {
        let l = (2 * node) + 1;
        let r = (2 * node) + 2;
        let mut largest = node;

        if l < self.len && self.arr[l] > self.arr[largest] {
            largest = l;
        }

        if r < self.len && self.arr[r] > self.arr[largest] {
            largest = r;
        }

        if largest != node {
            let tmp = self.arr[node];
            self.arr[node] = self.arr[largest];
            self.arr[largest] = tmp;
            self.heapify(largest);
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.len == self.arr.len() {
            self.len += 1;
            self.arr.push(val);
            self.percolate_up(self.len - 1);
        } else {
            self.arr[self.len] = val;
            self.len += 1;
            self.percolate_up(self.len - 1);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        let tmp = self.arr[0];
        self.arr[0] = self.arr[self.len - 1];
        self.arr[self.len - 1] = tmp;

        let result = Some(self.arr[self.len - 1]);
        self.len -= 1;
        self.heapify(0);

        return result;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::BinaryHeap;

    #[test]
    pub fn test_max_heap() {
        let mut max_heap1 = MaxHeap::new();
        let mut max_heap2 = BinaryHeap::new();
        let mut rng = rand::thread_rng();
        let mut size = 0;

        for _ in 0..10 {
            let add_count = rng.gen_range(1..=10);
            for _ in 0..add_count {
                let val = rng.gen_range(i32::MIN..=i32::MAX);
                max_heap1.push(val);
                max_heap2.push(val);
            }
            size += add_count;

            let remove_count = rng.gen_range(1..=size);
            for _ in 0..remove_count {
                assert_eq!(max_heap1.pop(), max_heap2.pop());
            }
            size -= remove_count;
        }
    }
}
