// Time taken: 21:07, 21:11 -> Wrong, 21:18 -> Acc
struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.chars().collect::<Vec<char>>();
        let len = sentence.len();
        let mut idx = 1;

        while idx < sentence.len() {
            if sentence[idx - 1] == ' ' {
                if sentence[idx] != sentence[idx - 2] {
                    return false;
                }
            }
            idx += 1;
        }

        return sentence[0] == sentence[len - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()),
            true
        );
        assert_eq!(Solution::is_circular_sentence("eetcode".to_string()), true);
        assert_eq!(
            Solution::is_circular_sentence("Leetcode is cool".to_string()),
            false
        );
        assert_eq!(
            Solution::is_circular_sentence("MuFoevIXCZzrpXeRmTssj lYSW U jM".to_string()),
            false
        );
    }
}
