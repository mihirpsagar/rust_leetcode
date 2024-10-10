// Time taken: 23:25, 23:27 -> Acc
struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            let mut n = num;
            let mut count = 0;
            while n > 0 {
                n /= 10;
                count += 1;
            }

            if count % 2 == 0 {
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
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
