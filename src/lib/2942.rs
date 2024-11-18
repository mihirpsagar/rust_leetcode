// Time taken: 16:29, 16:30 -> Acc
struct Solution {}

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = Vec::new();

        for (idx, word) in words.iter().enumerate() {
            for ch in word.chars() {
                if ch == x {
                    result.push(idx as i32);
                    break;
                }
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
            Solution::find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            [0, 1]
        );
    }
}
