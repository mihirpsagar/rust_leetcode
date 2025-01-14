// Time taken: 13:39, 13:58 -> Wrong, 14:05 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<(i32, Reverse<i32>, char)>> = BinaryHeap::new();
        let mut arr = vec![0; 26];
        let a = 'A' as u8;
        let mut result = 0;

        for task in tasks {
            arr[(task as u8 - a) as usize] += 1;
        }

        for (idx, &val) in arr.iter().enumerate() {
            if val != 0 {
                min_heap.push(Reverse((0, Reverse(val), (idx as u8 + a) as char)));
            }
        }

        while !min_heap.is_empty() {
            if min_heap.peek().unwrap().0 .0 > 0 {
                min_heap = Self::reduce_all(min_heap);
            } else {
                let mut node = min_heap.pop().unwrap();
                println!("{}", node.0 .2);
                min_heap = Self::reduce_all(min_heap);
                if node.0 .1 .0 > 1 {
                    node.0 .1 .0 -= 1;
                    node.0 .0 = n;
                    min_heap.push(node);
                }
            }
            result += 1;
        }

        return result;
    }

    fn reduce_all(
        mut min_heap: BinaryHeap<Reverse<(i32, Reverse<i32>, char)>>,
    ) -> BinaryHeap<Reverse<(i32, Reverse<i32>, char)>> {
        let mut new_min_heap = BinaryHeap::new();
        while let Some(mut node) = min_heap.pop() {
            if node.0 .0 > 0 {
                node.0 .0 -= 1;
            }
            new_min_heap.push(node);
        }

        return new_min_heap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1),
            6
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3),
            10
        );
        assert_eq!(
            Solution::least_interval(vec!['B', 'C', 'D', 'A', 'A', 'A', 'A', 'G'], 1),
            8
        );
    }
}
