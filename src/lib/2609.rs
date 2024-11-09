// Time taken: 10:56, 11:00, 11:03 -> Acc
struct Solution {}

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut zero_count = 0;
        let mut one_count = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut result = 0;

        while idx < s.len() {
            if s[idx] == '0' {
                if one_count > 0 {
                    result = std::cmp::max(result, std::cmp::min(zero_count, one_count) * 2);
                    zero_count = 1;
                    one_count = 0;
                } else {
                    zero_count += 1;
                }
            } else {
                one_count += 1;
            }
            idx += 1;
        }

        result = std::cmp::max(result, std::cmp::min(zero_count, one_count) * 2);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01000111".to_string()),
            6
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("00111".to_string()),
            4
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("111".to_string()),
            0
        );
    }
}
