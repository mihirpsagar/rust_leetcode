// Time taken: 09:45, 09:48 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            result += Self::get_num(num);
        }

        return result;
    }

    pub fn get_num(num: i32) -> i32 {
        let mut len = 0;
        let mut max_digit = 0;
        let mut val = num;

        while val > 0 {
            max_digit = std::cmp::max(max_digit, val % 10);
            len += 1;
            val /= 10;
        }

        let mut result = 0;
        for _ in 0..len {
            result = (result * 10) + max_digit;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
