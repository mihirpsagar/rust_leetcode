// Time taken: 09:49, 09:52 -> Acc
struct Solution {}

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();

        let mut idx1 = 0;
        while idx1 < s.len() - 1 {
            let ch1 = s[idx1];
            let ch2 = s[idx1 + 1];

            let mut idx2 = s.len() - 1;
            loop {
                if s[idx2] == ch1 && s[idx2 - 1] == ch2 {
                    return true;
                }
                if idx2 == 1 {
                    break;
                }
                idx2 -= 1;
            }

            idx1 += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_substring_present("leetcode".to_string()), true);
        assert_eq!(Solution::is_substring_present("abcba".to_string()), true);
        assert_eq!(Solution::is_substring_present("abcd".to_string()), false);
    }
}
