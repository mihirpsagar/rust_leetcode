// Time taken: 17:58, 18:00 -> Acc
struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut prev = s[0].to_ascii_lowercase();
        let mut idx = 1;

        while idx < s.len() {
            if s[idx].to_ascii_lowercase() != prev {
                result += 1;
                prev = s[idx].to_ascii_lowercase();
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_string()), 2);
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_string()), 0);
    }
}
