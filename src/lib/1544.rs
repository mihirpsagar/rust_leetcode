// Time taken: 19:05, 19:09 -> Acc
struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();

        'outer: loop {
            let mut idx = 0;
            let mut removed = false;
            'inner: while idx < s.len() - 1 {
                if s[idx] != s[idx + 1]
                    && s[idx].to_ascii_lowercase() == s[idx + 1].to_ascii_lowercase()
                {
                    s.remove(idx);
                    s.remove(idx);
                    removed = true;
                    break 'inner;
                }
                idx += 1;
            }
            if !removed || s.len() == 0 {
                break 'outer;
            }
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::make_good("leEeetcode".to_string()), "leetcode");
        assert_eq!(Solution::make_good("abBAcC".to_string()), "");
        assert_eq!(Solution::make_good("s".to_string()), "s");
    }
}
