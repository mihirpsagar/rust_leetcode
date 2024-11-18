use std::collections::HashMap;

// Time taken: 17:51, 17:56 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut map = HashMap::new();
        let mut count = 7;
        let mut result = 0;

        for ch in word.chars() {
            if let Some(val) = map.get(&ch) {
                result += *val;
            } else {
                count += 1;
                map.insert(ch, count / 8);
                result = result + (count / 8);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
        assert_eq!(Solution::minimum_pushes("xycdefghij".to_string()), 12);
    }
}
