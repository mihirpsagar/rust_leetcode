// Time taken: 11:32, 11:33 -> Acc
struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = Vec::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                result.pop();
            } else {
                result.push(ch);
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc");
        assert_eq!(Solution::clear_digits("cb34".to_string()), "");
    }
}
