// Time taken: 13:01, 13:06, 13:13 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut idx = 0;
        let mut count = 0;
        let mut result = String::new();
        let s = s.chars().collect::<Vec<char>>();

        let mut inner = String::new();
        while idx < s.len() {
            inner += &s[idx].to_string();
            if s[idx] == '(' {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                inner.remove(0);
                inner.remove(inner.len() - 1);
                result += &inner;
                inner = String::new();
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
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()"
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
