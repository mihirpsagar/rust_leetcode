// Time taken: 17:31, 17:36 -> Acc
struct Solution {}

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result = Vec::new();
        let mut prev_ch = 'a';
        let zero_ascii = '0' as u8;

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                let new_ch = (prev_ch as u8 + (ch as u8 - zero_ascii)) as char;
                result.push(new_ch);
            } else {
                prev_ch = ch;
                result.push(ch);
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::replace_digits("a1c1e1".to_string()), "abcdef");
        assert_eq!(
            Solution::replace_digits("a1b2c3d4e".to_string()),
            "abbdcfdhe"
        );
    }
}
