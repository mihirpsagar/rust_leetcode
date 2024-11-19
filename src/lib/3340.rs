// Time taken: 16:17, 16:19 -> Acc
struct Solution {}

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;
        let mut even = true;

        for ch in num.chars() {
            if even {
                even_sum = even_sum + (ch as u8 - '0' as u8) as u32;
            } else {
                odd_sum = odd_sum + (ch as u8 - '0' as u8) as u32;
            }
            even = !even;
        }

        return even_sum == odd_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_balanced("1234".to_string()), false);
        assert_eq!(Solution::is_balanced("24123".to_string()), true);
    }
}
