// Time taken: 21:27, 21:30 -> Acc
struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = Vec::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut prev = s[0];
        let mut count = 1;
        let mut idx = 1;
        result.push(s[0]);

        while idx < s.len() {
            if s[idx] == prev {
                count += 1;
                if count < 3 {
                    result.push(s[idx]);
                }
            } else {
                count = 1;
                result.push(s[idx]);
                prev = s[idx];
            }
            idx += 1;
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_string()),
            "leetcode"
        );
        assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa");
        assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab");
    }
}
