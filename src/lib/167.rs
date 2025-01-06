// Time taken: 20:43, 20:47 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];
            match sum.cmp(&target) {
                Ordering::Equal => {
                    return vec![(left + 1) as i32, (right + 1) as i32];
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Less => {
                    left += 1;
                }
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), [1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), [1, 2]);
    }
}
