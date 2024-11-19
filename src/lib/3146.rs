use std::collections::HashMap;

// Time taken: 11:15, 11:18 -> Acc
struct Solution {}

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = HashMap::new();

        for (idx, ch) in s.chars().enumerate() {
            map.insert(ch, idx);
        }

        for (idx, ch) in t.chars().enumerate() {
            if let Some(val) = map.get_mut(&ch) {
                *val = idx.abs_diff(*val);
            }
        }

        let mut result = 0;
        for val in map.values() {
            result += *val;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
