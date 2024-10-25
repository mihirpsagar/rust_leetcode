// Time taken: 20:40, 20:45 -> Wrong, 20:48 -> Acc
struct Solution {}

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < s.len() {
            if idx2 >= words.len() {
                break;
            }

            let word = words[idx2].chars().collect::<Vec<char>>();
            let mut idx3 = 0;
            while idx3 < word.len() && idx1 < s.len() {
                if s[idx1] != word[idx3] {
                    return false;
                }
                idx1 += 1;
                idx3 += 1;
            }

            if idx1 == s.len() && idx3 != word.len() {
                return false;
            }

            idx2 += 1;
        }

        return idx1 == s.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "apples".to_string()
                ]
            ),
            true
        );
        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "apples".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string()
                ]
            ),
            false
        );
        assert_eq!(
            Solution::is_prefix_string(
                "a".to_string(),
                vec!["aa".to_string(), "aaaa".to_string(), "banana".to_string()]
            ),
            false
        );
    }
}
