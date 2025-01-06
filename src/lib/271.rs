// Time taken: 11:56, 12:17, 12:35

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn encode(arr: Vec<String>) -> String {
        let mut result = Vec::new();
        for str in arr {
            let str = str.chars().collect::<Vec<char>>();
            Self::add_len(&mut result, str.len());
            result.push('#');
            for ch in str {
                result.push(ch);
            }
        }

        return result.iter().collect();
    }

    pub fn add_len(arr: &mut Vec<char>, len: usize) {
        let mut val = len;
        let mut stack = vec![];
        while val > 0 {
            stack.push(('0' as u8 + (val % 10) as u8) as char);
            val /= 10;
        }
        while let Some(val) = stack.pop() {
            arr.push(val);
        }
    }

    pub fn decode(str: String) -> Vec<String> {
        let mut result = Vec::new();
        let str = str.chars().collect::<Vec<char>>();
        let mut idx = 0;

        while idx < str.len() {
            let mut len = 0;
            let mut word = Vec::new();
            while str[idx] != '#' {
                len = (len * 10) + (str[idx] as u8 - '0' as u8) as usize;
                idx += 1;
            }

            idx += 1;
            for _ in 0..len {
                word.push(str[idx]);
                idx += 1;
            }

            result.push(word.iter().collect());
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::decode(Solution::encode(vec![
                "neet".to_string(),
                "code".to_string(),
                "love".to_string(),
                "you".to_string()
            ])),
            ["neet", "code", "love", "you"]
        );
        assert_eq!(
            Solution::decode(Solution::encode(vec![
                "we".to_string(),
                "say".to_string(),
                ":".to_string(),
                "yes".to_string()
            ])),
            ["we", "say", ":", "yes"]
        );
    }
}
