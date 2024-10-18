// Time taken: 22:29, 22:31 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        return n - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_matches(7), 6);
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
