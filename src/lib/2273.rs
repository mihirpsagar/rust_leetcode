use std::collections::HashMap;

// Time taken: 02:39, 02:44 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut idx = 1;

        result.push(words[0].clone());

        while idx < words.len() {
            if !Self::is_anagram(&result[result.len() - 1], &words[idx]) {
                result.push(words[idx].clone());
            }
            idx += 1;
        }

        return result;
    }

    pub fn is_anagram(word1: &String, word2: &String) -> bool {
        let mut map = HashMap::new();

        for ch in word1.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in word2.chars() {
            *map.entry(ch).or_insert(0) -= 1;
        }

        for &val in map.values() {
            if val != 0 {
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
            Solution::remove_anagrams(vec![
                "abba".to_string(),
                "baba".to_string(),
                "bbaa".to_string(),
                "cd".to_string(),
                "cd".to_string()
            ]),
            ["abba", "cd"]
        );
        assert_eq!(
            Solution::remove_anagrams(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]),
            ["a", "b", "c", "d", "e"]
        );
    }
}
