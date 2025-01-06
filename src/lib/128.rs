// Time taken: 20:20, 20:58 -> Acc

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut arr_set = HashSet::new();
        let mut seen_set = HashSet::new();

        for num in nums {
            arr_set.insert(num);
        }

        for num in arr_set.iter() {
            if seen_set.contains(num) {
                continue;
            }

            let mut start = *num;
            while arr_set.contains(&(start - 1)) {
                start -= 1;
            }

            let mut count = 0;
            while arr_set.contains(&start) {
                count += 1;
                seen_set.insert(start);
                start += 1;
            }

            result = std::cmp::max(result, count);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
