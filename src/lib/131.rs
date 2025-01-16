// Time taken: 23:37, 23:48, 23:55 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let s = s.chars().collect::<Vec<char>>();
        Self::partition_rec(&s, 0, 0, &mut Vec::new(), &mut result);
        return result;
    }

    pub fn partition_rec(
        s: &Vec<char>,
        start: usize,
        end: usize,
        mut curr: &mut Vec<String>,
        mut result: &mut Vec<Vec<String>>,
    ) {
        // Base condition
        let mut count = 0;
        for word in curr.iter() {
            count += word.len();
        }
        if count == s.len() {
            result.push(curr.clone());
        }

        // Base condition
        if start >= s.len() || end >= s.len() {
            return;
        }

        // Partition at start to end
        if Self::is_palindrome(&s, start, end) {
            let mut new_str = String::new();
            let mut idx = start;
            while idx <= end {
                new_str.push(s[idx]);
                idx += 1;
            }
            curr.push(new_str);

            Self::partition_rec(&s, end + 1, end + 1, &mut curr, &mut result);

            curr.pop();
        }

        // Do not partition
        if end + 1 < s.len() {
            Self::partition_rec(&s, start, end + 1, &mut curr, &mut result);
        }
    }

    pub fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
        let mut idx1 = start;
        let mut idx2 = end;
        while idx1 < idx2 {
            if s[idx1] != s[idx2] {
                return false;
            }
            idx1 += 1;
            idx2 -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
        assert_eq!(Solution::partition("a".to_string()), vec![["a"]]);
    }
}
