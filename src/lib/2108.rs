// Time taken: 20:54, 20:56 -> Acc
struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            let word_vec = word.chars().collect::<Vec<char>>();
            if Self::is_palindrome(&word_vec) {
                return word;
            }
        }

        return String::new();
    }

    pub fn is_palindrome(arr: &Vec<char>) -> bool {
        let mut left = 0;
        let mut right = arr.len() - 1;

        while left < right {
            if arr[left] != arr[right] {
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
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ]),
            "ada"
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            "racecar"
        );
        assert_eq!(
            Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
            ""
        );
    }
}
