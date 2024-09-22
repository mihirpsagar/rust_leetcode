use std::{cmp::Reverse, collections::BinaryHeap};

// Time taken: 13:45, 13:50, 14:10 -> Acc
struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let n_max = 3;

        for num in nums {
            if min_heap.iter().filter(|&x| x.0 == num).count() == 1 {
                continue;
            }

            if min_heap.len() < n_max {
                min_heap.push(Reverse(num));
                continue;
            }

            if min_heap.peek().unwrap().0 < num {
                min_heap.pop();
                min_heap.push(Reverse(num));
            }
        }

        if min_heap.len() == 2 {
            min_heap.pop();
        }

        return min_heap.peek().unwrap().0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
