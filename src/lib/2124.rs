// Time taken: 21:04, 21:05 -> Acc
struct Solution {}

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut b_found = false;

        for ch in s.chars() {
            if ch == 'b' {
                b_found = true;
            } else {
                if b_found {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_string("aaabbb".to_string()), true);
        assert_eq!(Solution::check_string("abab".to_string()), false);
        assert_eq!(Solution::check_string("bbb".to_string()), true);
    }
}
