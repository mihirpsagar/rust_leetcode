use std::collections::HashSet;

// Time taken: 21:04, 21:08 -> Acc
struct Solution {}

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut set = HashSet::new();
        let words = text.split(' ').collect::<Vec<&str>>();
        let mut result = 0;

        for ch in broken_letters.chars() {
            set.insert(ch);
        }

        for word in words {
            let mut valid = true;
            for ch in word.chars() {
                if set.contains(&ch) {
                    valid = false;
                    break;
                }
            }
            if valid {
                result += 1;
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
            Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "e".to_string()),
            0
        );
    }
}
