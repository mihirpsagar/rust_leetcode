// Time taken: 12:32, 12:41 -> Wrong, 12:42 -> Acc
struct Solution {}

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            if s[idx] == 'B' {
                if let Some(ch) = stack.pop() {
                    if ch != 'A' {
                        stack.push(ch);
                        stack.push(s[idx]);
                    }
                } else {
                    stack.push(s[idx]);
                }
            } else if s[idx] == 'D' {
                if let Some(ch) = stack.pop() {
                    if ch != 'C' {
                        stack.push(ch);
                        stack.push(s[idx]);
                    }
                } else {
                    stack.push(s[idx]);
                }
            } else {
                stack.push(s[idx]);
            }
            idx += 1;
        }

        return stack.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
        assert_eq!(Solution::min_length("ACBBD".to_string()), 5);
        assert_eq!(Solution::min_length("D".to_string()), 1);
    }
}
