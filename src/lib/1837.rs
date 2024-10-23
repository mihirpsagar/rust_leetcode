// Time taken: 17:24, 17:30 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut num = n;
        let mut result = 0;

        while num > 0 {
            result = result + (num % k);
            num /= k;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_base(34, 6), 9);
        assert_eq!(Solution::sum_base(10, 10), 1);
    }
}
