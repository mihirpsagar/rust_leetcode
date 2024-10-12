// Time taken: 13:14, 13:16 -> Acc
struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut result = 1;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 1;
        let mut prev_ch = s[0];
        let mut count = 1;

        while idx < s.len() {
            if s[idx] == prev_ch {
                count += 1;
                if count > result {
                    result = count;
                }
            } else {
                count = 1;
                prev_ch = s[idx];
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
        assert_eq!(Solution::max_power("leetcode".to_string()), 2);
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    }
}
