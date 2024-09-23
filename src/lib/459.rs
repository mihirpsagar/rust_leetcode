// Time taken: 15:31, 15:45 -> Acc
struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut lps: Vec<usize> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        let mut len = 0;
        let mut idx = 1;

        lps.push(0);
        while idx < s.len() {
            if s[idx] == s[len] {
                len += 1;
                lps.push(len);
                idx += 1;
            } else {
                if len > 0 {
                    len = lps[len - 1];
                } else {
                    lps.push(0);
                    idx += 1;
                }
            }
        }

        let num = lps[s.len() - 1];

        return num != 0 && s.len() % (s.len() - num) == 0;
    }

    // pub fn repeated_substring_pattern(s: String) -> bool {
    //     // let str = s.clone() + &s;
    //     let str = format!("{}{}", &s, &s);
    //     let str = &str[1..str.len() - 1];

    //     return str.contains(&s);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abab")),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("aba")),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abcabcabcabc")),
            true
        );
    }
}
