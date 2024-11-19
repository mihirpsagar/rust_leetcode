// Time taken: 11:29, 11:31 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut result = 0;
        let mut curr = 0;
        for ch in s.chars() {
            if ch == 'E' {
                curr += 1;
                result = std::cmp::max(result, curr);
            } else {
                curr -= 1;
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
        assert_eq!(Solution::minimum_chairs("EEEEEEE".to_string()), 7);
        assert_eq!(Solution::minimum_chairs("ELELEEL".to_string()), 2);
        assert_eq!(Solution::minimum_chairs("ELEELEELLL".to_string()), 3);
    }
}
