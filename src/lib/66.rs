// Time taken: 15:06, 15:11
struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut remainder: i32 = 1;

        for digit in digits.iter().rev() {
            result.push((digit + remainder) % 10);
            remainder = (digit + remainder) / 10;
        }

        if remainder > 0 {
            result.push(remainder);
        }

        result.reverse();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
