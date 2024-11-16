// Time taken: 21:09, 21:12 -> Acc

struct Solution {}

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = Vec::new();
        for word in words {
            let word = word.chars().collect::<Vec<char>>();
            let mut new_word = Vec::new();
            let mut idx = 0;
            while idx < word.len() {
                if word[idx] == separator {
                    if !new_word.is_empty() {
                        result.push(new_word.iter().collect());
                        new_word.clear();
                    }
                } else {
                    new_word.push(word[idx]);
                }
                idx += 1;
            }
            if !new_word.is_empty() {
                result.push(new_word.iter().collect());
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
            Solution::split_words_by_separator(
                vec![
                    "one.two.three".to_string(),
                    "four.five".to_string(),
                    "six".to_string()
                ],
                '.'
            ),
            ["one", "two", "three", "four", "five", "six"]
        );
        assert_eq!(
            Solution::split_words_by_separator(
                vec!["$easy$".to_string(), "$problem$".to_string()],
                '$'
            ),
            ["easy", "problem"]
        );
        assert_eq!(
            Solution::split_words_by_separator(vec!["|||".to_string()], '|').is_empty(),
            true
        );
    }
}
