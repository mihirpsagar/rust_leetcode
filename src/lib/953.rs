use std::collections::HashMap;

// Time taken: 13:14, 13:19, 13:26, 13:33 -> Acc
struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = HashMap::new();

        for (idx, ch) in order.chars().enumerate() {
            map.insert(ch, idx);
        }

        let mut prev_word = Vec::new();

        for word in words {
            let word: Vec<char> = word.chars().collect();
            if prev_word.is_empty() {
                prev_word = word.clone();
            } else {
                let mut idx = 0;
                let len = std::cmp::max(prev_word.len(), word.len());
                while idx < len {
                    if let Some(ch1) = prev_word.get(idx) {
                        if let Some(ch2) = word.get(idx) {
                            let &val1 = map.get(ch1).unwrap();
                            let &val2 = map.get(ch2).unwrap();
                            if val1 < val2 {
                                break;
                            } else {
                                if val1 > val2 {
                                    return false;
                                }
                            }
                        } else {
                            return false;
                        }
                    } else {
                        break;
                    }
                    idx += 1;
                }
                prev_word = word.clone();
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
            Solution::is_alien_sorted(
                vec!["hello".to_string(), "leetcode".to_string()],
                "hlabcdefgijkmnopqrstuvwxyz".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_alien_sorted(
                vec!["word".to_string(), "world".to_string(), "row".to_string()],
                "worldabcefghijkmnpqstuvxyz".to_string()
            ),
            false
        );
        assert_eq!(
            Solution::is_alien_sorted(
                vec!["apple".to_string(), "app".to_string()],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            false
        );
    }
}
