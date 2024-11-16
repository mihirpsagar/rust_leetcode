// Time taken: 12:55, 12:58 -> Acc
struct Solution {}

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] < s[right] {
                s[right] = s[left];
            } else {
                s[left] = s[right];
            }
            left += 1;
            right = right.saturating_sub(1);
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::make_smallest_palindrome("egcfe".to_string()),
            "efcfe"
        );
        assert_eq!(
            Solution::make_smallest_palindrome("abcd".to_string()),
            "abba"
        );
        assert_eq!(
            Solution::make_smallest_palindrome("seven".to_string()),
            "neven"
        );
    }
}
