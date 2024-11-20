use std::collections::HashSet;

// Time taken: 17:32, 17:39, 17:46 -> Wrong, 17:52 -> Acc
struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut result = 0;
        let mut left = 0;
        let mut right = 0;

        while right < s.len() {
            if set.contains(&s[right]) {
                while set.contains(&s[right]) {
                    set.remove(&s[left]);
                    left += 1;
                }
            }
            set.insert(s[right]);
            result = std::cmp::max(result, right - left + 1);

            right += 1;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("aabaab!bb".to_string()),
            3
        );
    }
}
