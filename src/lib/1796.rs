// Time taken: 23:49, 23:52 -> Acc
struct Solution {}

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut result = (-1, -1);

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                let digit = i32::from_str_radix(&String::from(ch), 10).unwrap();
                if digit == result.0 {
                    continue;
                } else if digit > result.0 {
                    result.1 = result.0;
                    result.0 = digit;
                } else {
                    if digit > result.1 {
                        result.1 = digit;
                    }
                }
            }
        }

        return result.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    }
}
