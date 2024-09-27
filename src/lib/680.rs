// Time taken: 00:04, 00:07, 00:13 -> Wrong, 00:21 -> Acc
struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();

        if s.len() < 3 {
            return true;
        }

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if s[left] != s[right] {
                return Self::is_palindrome(&s, left, right - 1)
                    || Self::is_palindrome(&s, left + 1, right);
            }
            left += 1;
            right -= 1;
        }

        return true;
    }

    fn is_palindrome(s: &Vec<char>, left: usize, right: usize) -> bool {
        let mut left = left;
        let mut right = right;
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
        assert_eq!(
            Solution::valid_palindrome("ebcbbececabbacecbbcbe".to_string()),
            true
        );
    }
}
