// Time taken: 16:13, 16:16 -> Acc
struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut result = 1;
        let mut idx = 1;
        let word = word.chars().collect::<Vec<char>>();

        let mut curr = 1;
        while idx < word.len() {
            if word[idx] == word[idx - 1] {
                curr += 1;
            } else {
                result = result + (curr - 1);
                curr = 1;
            }
            idx += 1;
        }

        if curr > 1 {
            result = result + (curr - 1);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
        assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
        assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
    }
}
