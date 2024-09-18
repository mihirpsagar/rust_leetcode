use std::collections::{HashMap, HashSet};

// Time taken: 16:13, 16:21 -> Wrong, 16:27 -> Acc
struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut result = true;
        let mut hash_map = HashMap::new();
        let mut hash_set = HashSet::new();
        let p_iter = pattern.chars();
        let mut s_iter = s.split(" ");

        for p_ch in p_iter {
            let mut s_word = "";
            match s_iter.next() {
                None => {
                    result = false;
                    break;
                }
                Some(val) => {
                    s_word = val;
                }
            }

            match hash_map.get(&p_ch) {
                None => {
                    if hash_set.insert(s_word) {
                        hash_map.insert(p_ch, s_word);
                    } else {
                        result = false;
                        break;
                    }
                }
                Some(&val) => {
                    if val != s_word {
                        result = false;
                        break;
                    }
                }
            }
        }

        if s_iter.next().is_some() {
            result = false;
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
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("aaa"), String::from("aa aa aa aa")),
            false
        );
    }
}
