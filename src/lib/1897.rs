use std::collections::HashMap;

// Time taken: 23:17, 23:19 -> Acc
struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut map = HashMap::new();
        let len = words.len();

        for word in words {
            for ch in word.chars() {
                *map.entry(ch).or_insert(0) += 1;
            }
        }

        for val in map.values() {
            if val % len != 0 {
                return false;
            }
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
            Solution::make_equal(vec![
                "abc".to_string(),
                "aabc".to_string(),
                "bc".to_string()
            ]),
            true
        );
        assert_eq!(
            Solution::make_equal(vec!["ab".to_string(), "a".to_string()]),
            false
        );
    }
}
