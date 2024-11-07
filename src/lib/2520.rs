// Time taken: 23:25, 23:27 -> Acc
struct Solution {}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut result = 0;
        let mut val = num;

        while val > 0 {
            let digit = val % 10;
            if num % digit == 0 {
                result += 1;
            }
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
        assert_eq!(Solution::count_digits(7), 1);
        assert_eq!(Solution::count_digits(121), 2);
        assert_eq!(Solution::count_digits(1248), 4);
    }
}
