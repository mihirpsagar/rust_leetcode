// Time taken: 12:41, 12:54 -> Acc
struct Solution {}

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut left = 0;
        let mut right = s.len() as i32;
        let mut result: Vec<i32> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        let mut idx = 0;

        while idx < s.len() {
            if s[idx] == 'I' {
                result.push(left);
                left += 1;
            } else {
                result.push(right);
                right -= 1;
            }
            idx += 1;
        }

        result.push(left);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::di_string_match("IDID".to_string()),
            [0, 4, 1, 3, 2]
        );
        assert_eq!(Solution::di_string_match("III".to_string()), [0, 1, 2, 3]);
        assert_eq!(Solution::di_string_match("DDI".to_string()), [3, 2, 0, 1]);
    }
}
