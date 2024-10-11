// Time taken: 20:36, 20:38 -> Wrong, 20:45 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut changed = false;
        let mut result = 0;
        let mut reverse_num = 0;
        let mut n = num;

        while n > 0 {
            reverse_num = (reverse_num * 10) + n % 10;
            n /= 10;
        }

        let mut n = reverse_num;
        while n > 0 {
            let mut digit = n % 10;
            if digit == 6 && !changed {
                changed = true;
                digit = 9;
            }
            result = (result * 10) + digit;
            n /= 10;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
        assert_eq!(Solution::maximum69_number(9996), 9999);
        assert_eq!(Solution::maximum69_number(9999), 9999);
        assert_eq!(Solution::maximum69_number(669), 969);
    }
}
