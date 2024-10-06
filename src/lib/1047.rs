// Time taken: 19:56, 20:01 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        let s = s.chars().collect::<Vec<char>>();

        let mut idx = 0;
        while idx < s.len() {
            if stack.is_empty() {
                stack.push(s[idx]);
            } else {
                if stack[stack.len() - 1] == s[idx] {
                    stack.pop();
                } else {
                    stack.push(s[idx]);
                }
            }
            idx += 1;
        }

        return stack.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
    }
}
