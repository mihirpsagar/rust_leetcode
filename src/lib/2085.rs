use std::collections::HashMap;

// Time taken: 10:10, 10:13, 10:16 -> Acc
struct Solution {}

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut result = 0;

        for word in words1 {
            *map1.entry(word).or_insert(0) += 1;
        }

        for word in words2 {
            *map2.entry(word).or_insert(0) += 1;
        }

        for (key, val) in map1 {
            if val == 1 {
                if let Some(&val2) = map2.get(&key) {
                    if val2 == 1 {
                        result += 1;
                    }
                }
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
        assert_eq!(
            Solution::count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            ),
            0
        );
        assert_eq!(
            Solution::count_words(
                vec!["a".to_string(), "ab".to_string()],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            ),
            1
        );
    }
}
