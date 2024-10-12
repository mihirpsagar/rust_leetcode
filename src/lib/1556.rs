// Time taken: 19:52, 19:54 -> Wrong, 19:56 -> Wrong, 19:58 -> Acc
struct Solution {}

impl Solution {
    pub fn thousand_separator(mut n: i32) -> String {
        let mut result = Vec::new();
        let ascii_zero = '0' as u8;
        let mut count = 0;
        loop {
            if count != 0 && count % 3 == 0 {
                result.push('.');
            }
            result.push(((n % 10) as u8 + ascii_zero) as char);
            count += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }

        return result.iter().rev().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::thousand_separator(987), "987");
        assert_eq!(Solution::thousand_separator(1234), "1.234");
        assert_eq!(Solution::thousand_separator(123456789), "123.456.789");
        assert_eq!(Solution::thousand_separator(0), "0");
    }
}
