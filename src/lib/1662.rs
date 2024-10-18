// Time taken: 17:44, 17:48 -> Acc
struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut left_word = String::new();
        let mut right_word = String::new();

        for word in word1 {
            left_word.push_str(&word);
        }

        for word in word2 {
            right_word.push_str(&word);
        }

        return left_word == right_word;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["ab".to_string(), "c".to_string()],
                vec!["a".to_string(), "bc".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["a".to_string(), "cb".to_string()],
                vec!["ab".to_string(), "c".to_string()]
            ),
            false
        );
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
                vec!["abcddefg".to_string()]
            ),
            true
        );
    }
}
