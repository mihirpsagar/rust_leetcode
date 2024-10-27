// Time taken: 18:51, 18:53 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;

        while idx < s.len() {
            if s[idx] == 'X' {
                result += 1;
                idx += 3;
            } else {
                idx += 1;
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
        assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
        assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
        assert_eq!(Solution::minimum_moves("OOOO".to_string()), 0);
    }
}
