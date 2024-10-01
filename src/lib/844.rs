// Time taken: 21:20, 21:23 -> Acc
struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();

        for ch in s.chars() {
            if ch == '#' {
                stack1.pop();
            } else {
                stack1.push(ch);
            }
        }

        for ch in t.chars() {
            if ch == '#' {
                stack2.pop();
            } else {
                stack2.push(ch);
            }
        }

        if stack1.len() != stack2.len() {
            return false;
        }

        let mut idx = 0;
        while idx < stack1.len() {
            if stack1[idx] != stack2[idx] {
                return false;
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }
}
