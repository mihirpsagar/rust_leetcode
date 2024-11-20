// Time taken: 13:39, 13:46, 13:55 -> Acc
struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim().chars().collect::<Vec<char>>();
        let mut result: i32 = 0;
        let mut idx = 0;
        let mut is_positive = true;
        let mut non_zero_found = false;
        let zero = '0' as u8;

        if s.is_empty() {
            return 0;
        }

        if s[0] == '-' {
            is_positive = false;
            idx = 1;
        } else if s[0] == '+' {
            idx = 1;
        }

        while idx < s.len() {
            if s[idx] == '0' && !non_zero_found {
                idx += 1;
                continue;
            }
            if s[idx].is_ascii_digit() {
                non_zero_found = true;
                result = result.saturating_mul(10);
                if is_positive {
                    result = result.saturating_add((s[idx] as u8 - zero) as i32);
                } else {
                    result = result.saturating_sub((s[idx] as u8 - zero) as i32);
                }
            } else {
                break;
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
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
