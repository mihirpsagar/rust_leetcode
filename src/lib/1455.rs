// Time taken: 13:21, 13:27 -> Acc
struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let sentence = sentence.split_ascii_whitespace().collect::<Vec<&str>>();
        let search_word = search_word.chars().collect::<Vec<char>>();

        'outer: for (i, word) in sentence.iter().enumerate() {
            let source = word.chars().collect::<Vec<char>>();
            if source.len() < search_word.len() {
                continue 'outer;
            }

            let mut idx = 0;
            while idx < search_word.len() {
                if search_word[idx] != source[idx] {
                    continue 'outer;
                }
                idx += 1;
            }

            return i as i32 + 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
