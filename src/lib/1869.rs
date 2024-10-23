// Time taken: 18:23, 18:26 -> Acc
struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max_zero_length = 0;
        let mut max_one_length = 0;
        let s = s.chars().collect::<Vec<char>>();

        let mut curr_zero_length = 0;
        let mut curr_one_length = 0;
        let mut idx = 0;

        while idx < s.len() {
            if s[idx] == '0' {
                curr_zero_length += 1;
                if curr_zero_length > max_zero_length {
                    max_zero_length = curr_zero_length;
                }
                curr_one_length = 0;
            } else {
                curr_one_length += 1;
                if curr_one_length > max_one_length {
                    max_one_length = curr_one_length;
                }
                curr_zero_length = 0;
            }
            idx += 1;
        }

        return max_one_length > max_zero_length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
        assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
        assert_eq!(Solution::check_zero_ones("110100010".to_string()), false);
    }
}
