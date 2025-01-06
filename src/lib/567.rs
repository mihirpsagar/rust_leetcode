// Time taken: 15:52, 16:04 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

#[derive(PartialEq)]
enum State {
    Equal,
    Greater,
    Lesser,
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut arr1 = vec![0; 26];
        let a = 'a' as u8;
        for ch in s1.chars() {
            arr1[(ch as u8 - a) as usize] += 1;
        }

        let s2 = s2.chars().collect::<Vec<char>>();
        let mut arr2 = vec![0; 26];
        let mut left = 0;
        let mut right = 0;
        while right < s2.len() {
            arr2[(s2[right] as u8 - a) as usize] += 1;
            match Self::is_valid(&arr1, &arr2) {
                State::Equal => {
                    return true;
                }
                State::Lesser => {}
                State::Greater => {
                    while Self::is_valid(&arr1, &arr2) == State::Greater {
                        arr2[(s2[left] as u8 - a) as usize] -= 1;
                        left += 1;
                    }
                }
            }
            right += 1;
        }

        return false;
    }

    pub fn is_valid(arr1: &Vec<i32>, arr2: &Vec<i32>) -> State {
        let mut idx = 0;
        let mut lesser = false;
        while idx < arr1.len() {
            if arr1[idx] < arr2[idx] {
                return State::Greater;
            } else if arr1[idx] > arr2[idx] {
                lesser = true;
            }
            idx += 1;
        }

        if lesser {
            return State::Lesser;
        } else {
            return State::Equal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }
}
