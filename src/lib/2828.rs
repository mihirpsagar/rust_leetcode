// Time taken: 23:04, 23:06 -> Acc
struct Solution {}

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut result = Vec::new();
        for word in words {
            let word = word.chars().collect::<Vec<char>>();
            result.push(word[0]);
        }
        return result.iter().collect::<String>() == s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_acronym(
                vec![
                    "alice".to_string(),
                    "bob".to_string(),
                    "charlie".to_string()
                ],
                "abc".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_acronym(vec!["an".to_string(), "apple".to_string()], "a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_acronym(
                vec![
                    "never".to_string(),
                    "gonna".to_string(),
                    "give".to_string(),
                    "up".to_string(),
                    "on".to_string(),
                    "you".to_string()
                ],
                "ngguoy".to_string()
            ),
            true
        );
    }
}
