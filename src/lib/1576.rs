// Time taken: 19:21, 19:38 -> Wrong, 19:43 -> Acc
struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut result = Vec::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let a_ascii = 'a' as u8;

        while idx < s.len() {
            if s[idx] != '?' {
                result.push(s[idx]);
                if idx != 0 {
                    if result[idx - 1] == result[idx] {
                        result[idx - 1] =
                            (((result[idx - 1] as u8 - a_ascii + 1) % 26) + a_ascii) as char;
                    }
                }
            } else {
                if idx == 0 {
                    result.push('a');
                } else {
                    let ch = (((result[idx - 1] as u8 - a_ascii + 1) % 26) + a_ascii) as char;
                    result.push(ch);
                }
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
        assert_eq!(Solution::modify_string("?zs".to_string()), "azs");
        assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvxw");
        assert_eq!(Solution::modify_string("???".to_string()), "abc");
    }
}
