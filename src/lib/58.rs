// Time taken: 14:56, 15:00
struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut curr_len: i32 = 0;

        for ch in s.chars() {
            if ch != ' ' {
                curr_len += 1;
            } else {
                if curr_len != 0 {
                    result = curr_len;
                    curr_len = 0;
                }
            }
        }

        if curr_len != 0 {
            result = curr_len;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
