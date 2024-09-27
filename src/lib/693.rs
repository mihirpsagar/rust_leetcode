// Time taken: 00:31, 00:35 -> Acc
struct Solution {}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut last_bit = if n % 2 == 0 { 1 } else { 0 };
        let mut n = n;

        while n > 0 {
            let bit = n % 2;
            if bit == last_bit {
                return false;
            }
            last_bit = bit;
            n /= 2;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::has_alternating_bits(5), true);
        assert_eq!(Solution::has_alternating_bits(7), false);
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}
