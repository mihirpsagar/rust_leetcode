// Time taken: 11:05, 11:09 -> Acc
struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        let mut has_vowel = false;
        let mut has_consonant = false;
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        if word.len() < 3 {
            return false;
        }

        let mut idx = 0;
        while idx < word.len() {
            let mut ch = word[idx];
            if ch.is_ascii_alphabetic() {
                ch = ch.to_ascii_lowercase();
                if vowels.contains(&ch) {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if ch.is_ascii_digit() {
                idx += 1;
                continue;
            } else {
                return false;
            }

            idx += 1;
        }

        return has_vowel && has_consonant;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_valid("234Adas".to_string()), true);
        assert_eq!(Solution::is_valid("b3".to_string()), false);
        assert_eq!(Solution::is_valid("a3$e".to_string()), false);
    }
}
