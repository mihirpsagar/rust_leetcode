// Time taken: 02:03, 02:07 -> Acc
struct Solution {}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut result = 0;

        for word in words {
            let word = word.chars().collect::<Vec<char>>();
            if Self::is_prefix(&s, &word) {
                result += 1;
            }
        }

        return result;
    }

    pub fn is_prefix(word: &Vec<char>, pattern: &Vec<char>) -> bool {
        let mut left = 0;
        let mut right = 0;

        while left < word.len() && right < pattern.len() {
            if word[left] != pattern[right] {
                return false;
            }
            left += 1;
            right += 1;
        }

        return right == pattern.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_prefixes(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()),
            2
        );
    }
}
