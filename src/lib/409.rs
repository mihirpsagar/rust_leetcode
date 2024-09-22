use std::collections::HashMap;

// Time taken: 13:28, 13:32, 13:38 -> Acc
struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut hash_map = HashMap::new();

        for ch in s.chars() {
            match hash_map.get(&ch) {
                None => {
                    hash_map.insert(ch, 1);
                }
                Some(&val) => {
                    hash_map.insert(ch, val + 1);
                }
            }
        }

        let mut result = 0;
        for (_, &val) in hash_map.iter() {
            if val > 1 {
                if val % 2 == 0 {
                    result += val;
                } else {
                    result += val - 1;
                }
            }
        }

        if result != s.len() {
            result += 1;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("abc")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("abcc")), 3);
    }
}
