// Time taken: 13:27, 13:29 -> Wrong, 13:30 -> Wrong, 13:33 -> Acc
struct Solution {}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut idx = 1;
        let s = s.chars().collect::<Vec<char>>();

        while idx < s.len() - 1 {
            if s[idx] == '0' {
                if s[idx + 1] == '1' {
                    return false;
                }
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
        assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
        assert_eq!(Solution::check_ones_segment("110".to_string()), true);
    }
}
