// Time taken: 01:05, 01:13 -> Acc
struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s = s.chars().collect::<Vec<char>>();
        let goal = goal.chars().collect::<Vec<char>>();
        let len = goal.len();

        let mut idx = 0;

        while idx < len {
            if goal[idx] == s[0] {
                let mut p1 = 0;
                let mut p2 = idx;
                let mut result = true;

                while p1 < len {
                    if s[p1] != goal[p2 % len] {
                        result = false;
                        break;
                    }
                    p1 += 1;
                    p2 += 1;
                }

                if result {
                    return true;
                }
            }
            idx += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "abced".to_string()),
            false
        );
    }
}
