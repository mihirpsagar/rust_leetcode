// Time taken: 17:11, 17:19, 17:35 -> Acc
struct Solution {}

impl Solution {}

struct OrderedStream {
    arr: Vec<String>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        let arr = vec![String::new(); n as usize];
        return Self { arr: arr, ptr: 0 };
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let mut result = Vec::new();
        let id_key = id_key as usize - 1;
        self.arr[id_key] = value;

        if self.ptr == id_key {
            let mut idx = id_key;
            while idx < self.arr.len() {
                if self.arr[idx].is_empty() {
                    break;
                }
                result.push(self.arr[idx].clone());
                idx += 1;
            }
            self.ptr = idx;
        }

        return result;
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
