// Time taken: 13:29, 13:35 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut digits = Vec::new();
        let mut val = x.abs();
        while val > 0 {
            digits.push(val % 10);
            val /= 10;
        }

        let mut idx = 0;
        let mut result: i32 = 0;
        while idx < digits.len() {
            if result.checked_mul(10).is_none() {
                return 0;
            }
            result *= 10;
            if result.checked_add(digits[idx]).is_none() {
                return 0;
            }
            result += digits[idx];
            idx += 1;
        }

        if x < 0 {
            if result.checked_mul(-1).is_none() {
                return 0;
            }
            result *= -1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
