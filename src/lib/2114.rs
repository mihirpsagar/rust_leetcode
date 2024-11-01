// Time taken: 20:58, 21:00 -> Acc
struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut result = 0;

        for sentence in sentences {
            let mut count = 1;

            for ch in sentence.chars() {
                if ch == ' ' {
                    count += 1;
                }
            }

            if count > result {
                result = count;
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
            Solution::most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::most_words_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string()
            ]),
            3
        );
    }
}
