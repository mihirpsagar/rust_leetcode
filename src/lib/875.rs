// Time taken: 14:19, 16:29 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let len = piles.len();
        let mut left = 1;
        let mut right = piles[0];

        let mut idx = 1;
        while idx < len {
            right = std::cmp::max(right, piles[idx]);
            idx += 1;
        }

        let mut result = right;
        right += 1;
        'outer: while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;
            idx = 0;
            while idx < len {
                count = count + (piles[idx] / mid);
                if piles[idx] % mid != 0 {
                    count += 1;
                }
                if count > h {
                    left = mid + 1;
                    continue 'outer;
                }
                idx += 1;
            }

            result = std::cmp::min(mid, result);
            right = mid;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
