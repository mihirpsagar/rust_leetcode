// Time taken: 23:21, 23:34 -> Wrong, 23:39 -> Acc
struct Solution {}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut result = 0;
        let arr = vec![11, 22, 33, 44, 55, 66, 77, 88, 99];

        for num in arr {
            if num >= low && num <= high {
                result += 1;
            }
        }

        let mut val = std::cmp::max(low, 1000);
        while val <= high {
            if Self::is_valid(val) {
                result += 1;
            }
            val += 1;
        }

        return result;
    }

    pub fn is_valid(num: i32) -> bool {
        let mut len = 0;
        let mut val = num;
        while val > 0 {
            val /= 10;
            len += 1;
        }

        if len % 2 != 0 {
            return false;
        }

        let mut num = num;
        let mut left_sum = 0;
        let mut right_sum = 0;
        for _ in 0..(len / 2) {
            right_sum = right_sum + (num % 10);
            num /= 10;
        }
        for _ in 0..(len / 2) {
            left_sum = left_sum + (num % 10);
            num /= 10;
        }

        return left_sum == right_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
        assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
        assert_eq!(Solution::count_symmetric_integers(100, 10000), 615);
    }
}
