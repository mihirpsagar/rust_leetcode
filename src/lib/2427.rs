// Time taken: 00:59, 01:00 -> Acc
struct Solution {}

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut result = 1;
        let min = std::cmp::min(a, b);

        for val in 2..=min {
            if a % val == 0 && b % val == 0 {
                result += 1;
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
        assert_eq!(Solution::common_factors(12, 6), 4);
        assert_eq!(Solution::common_factors(25, 30), 2);
    }
}
