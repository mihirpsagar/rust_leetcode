// Time taken: 14:35, 14:51 -> Wrong, 15:00 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut arr = vec![0; 26];
        let a = 'A' as u8;

        let mut left = 0;
        let mut right = 0;
        while right < s.len() {
            arr[(s[right] as u8 - a) as usize] += 1;
            if !Self::is_valid(&arr, k) {
                while !Self::is_valid(&arr, k) {
                    arr[(s[left] as u8 - a) as usize] -= 1;
                    left += 1;
                }
            } else {
                result = std::cmp::max(result, right - left + 1);
            }
            right += 1;
        }

        return result as i32;
    }

    pub fn is_valid(arr: &Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut max = 0;
        for &num in arr {
            sum += num;
            max = std::cmp::max(max, num);
        }

        sum -= max;

        return sum <= k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
        assert_eq!(Solution::character_replacement("AAAA".to_string(), 1), 4);
        assert_eq!(Solution::character_replacement("ABCD".to_string(), 0), 1);
        assert_eq!(Solution::character_replacement("ABBB".to_string(), 2), 4);
    }
}
