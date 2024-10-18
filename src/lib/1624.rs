use std::collections::HashMap;

// Time taken: 13:04, 13:09 -> Acc
struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut result: i32 = -1;
        let s = s.chars().collect::<Vec<char>>();

        let mut idx = 0;
        while idx < s.len() {
            if let Some(prev_idx) = map.get(&s[idx]) {
                let diff = (idx - prev_idx - 1) as i32;
                if result < diff {
                    result = diff;
                }
            } else {
                map.insert(s[idx], idx);
            }
            idx += 1;
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
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }
}
