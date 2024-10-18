// Time taken: 12:41, 12:43 -> Acc
struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut result = 0;
        let mut curr = 0;

        for ch in s.chars() {
            if ch == '(' {
                curr += 1;
                if curr > result {
                    result = curr;
                }
            } else {
                if ch == ')' {
                    curr -= 1;
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
        assert_eq!(Solution::max_depth("()(())((()()))".to_string()), 3);
    }
}
