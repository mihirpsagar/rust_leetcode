// Time taken: 14:13, 14:17 -> Acc
struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        return n > 0 && n.count_ones() == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
        assert_eq!(Solution::is_power_of_two(-16), false);
        assert_eq!(Solution::is_power_of_two(-2147483648), false);
    }
}
