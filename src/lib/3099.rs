// Time taken: 10:31, 10:32 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut val = x;
        let mut sum = 0;
        while val > 0 {
            sum = sum + (val % 10);
            val /= 10;
        }

        if x % sum == 0 {
            return sum;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }
}
