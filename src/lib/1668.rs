// Time taken: 17:55, 17:58 -> Wrong, 17:59 -> Acc
struct Solution {}

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let orig_word = word.clone();
        let mut word = word;
        let mut result = 0;

        loop {
            if sequence.contains(&word) {
                result += 1;
                word.push_str(&orig_word);
            } else {
                break;
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
            Solution::max_repeating("ababc".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ba".to_string()),
            1
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ac".to_string()),
            0
        );
        assert_eq!(
            Solution::max_repeating(
                "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(),
                "aaaba".to_string()
            ),
            5
        );
    }
}
