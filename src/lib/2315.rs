// Time taken: 13:27, 13:33 -> Acc
struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut is_valid = true;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut result = 0;

        while idx < s.len() {
            if s[idx] == '|' {
                is_valid = !is_valid;
            } else if s[idx] == '*' && is_valid {
                result += 1;
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
        assert_eq!(
            Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
            2
        );
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
        assert_eq!(
            Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()),
            5
        );
    }
}
