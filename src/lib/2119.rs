// Time taken: 21:02, 21:03 -> Acc
struct Solution {}

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        if num > 0 && num % 10 == 0 {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_same_after_reversals(526), true);
        assert_eq!(Solution::is_same_after_reversals(1800), false);
        assert_eq!(Solution::is_same_after_reversals(0), true);
    }
}
