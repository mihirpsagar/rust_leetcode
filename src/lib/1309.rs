// Time taken: 23:39, 23:46 -> Acc
struct Solution {}

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();
        let mut idx = 0;
        let s = s.chars().collect::<Vec<char>>();
        let a_ascii = 'a' as u8;

        while idx < s.len() {
            if idx + 2 < s.len() && s[idx + 2] == '#' {
                let val = u8::from_str_radix(&(s[idx].to_string() + &s[idx + 1].to_string()), 10)
                    .unwrap();
                result.push((val + a_ascii - 1) as char);
                idx += 3;
            } else {
                let val = u8::from_str_radix(&s[idx].to_string(), 10).unwrap();
                result.push((val + a_ascii - 1) as char);
                idx += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::freq_alphabets("10#11#12".to_string()), "jkab");
        assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz");
    }
}
