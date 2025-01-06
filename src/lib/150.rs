// Time taken: 21:16, 21:27 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            let arr = token.chars().collect::<Vec<char>>();

            if arr.len() == 1 {
                match arr[0] {
                    '+' => {
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        stack.push(val1 + val2);
                    }
                    '-' => {
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        stack.push(val1 - val2);
                    }
                    '*' => {
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        stack.push(val1 * val2);
                    }
                    '/' => {
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        stack.push(val1 / val2);
                    }
                    _ => {
                        stack.push(Self::convert_to_num(&arr));
                    }
                }
            } else {
                stack.push(Self::convert_to_num(&arr));
            }
        }

        return stack.pop().unwrap();
    }

    pub fn convert_to_num(arr: &Vec<char>) -> i32 {
        let mut result = 0;
        let mut is_neg = false;
        let mut idx = 0;

        if arr[0] == '-' {
            is_neg = true;
            idx = 1;
        }

        while idx < arr.len() {
            result *= 10;
            result = result + (arr[idx] as u8 - '0' as u8) as i32;
            idx += 1;
        }

        if is_neg {
            result *= -1;
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
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
    }
}
