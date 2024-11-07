// Time taken: 23:46, 23:48 -> Wrong, 23:53 -> Acc
struct Solution {}

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut result = 0;
        let mut val = n;
        let mut add = true;

        while val > 0 {
            val /= 10;
            add = !add;
        }
        add = !add;

        val = n;
        while val > 0 {
            let digit = val % 10;
            if add {
                result += digit;
            } else {
                result -= digit;
            }

            add = !add;
            val /= 10;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
        assert_eq!(Solution::alternate_digit_sum(111), 1);
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
        assert_eq!(Solution::alternate_digit_sum(10), 1);
    }
}
