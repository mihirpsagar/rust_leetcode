// Time taken: 21:02, 21:12 -> Wrong, 21:17 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut idx1 = 0;
        let mut idx2 = s.len() - 1;

        while idx1 < idx2 {
            if s[idx1] != s[idx2] {
                return 2;
            }
            idx1 += 1;
            idx2 -= 1;
        }

        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("ababb".to_string()), 2);
    }
}
