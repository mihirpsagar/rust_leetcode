use std::collections::HashMap;

// Time taken: 09:37, 09:40 -> Acc
struct Solution {}

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut map = HashMap::new();

        for ch in word1.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in word2.chars() {
            *map.entry(ch).or_insert(0) -= 1;
        }

        for &val in map.values() {
            if val > 3 || val < -3 {
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
            Solution::check_almost_equivalent("aaaa".to_string(), "bccb".to_string()),
            false
        );
        assert_eq!(
            Solution::check_almost_equivalent("abcdeef".to_string(), "abaaacc".to_string()),
            true
        );
        assert_eq!(
            Solution::check_almost_equivalent("cccddabba".to_string(), "babababab".to_string()),
            true
        );
    }
}
