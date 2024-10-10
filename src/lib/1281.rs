// Time taken: 23:13, 23:16 -> Acc
struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut digits = Vec::new();
        let mut n = n;

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let mut product = digits[0];
        let mut sum = digits[0];

        for num in digits.iter().skip(1) {
            product *= num;
            sum += num;
        }

        return product - sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
