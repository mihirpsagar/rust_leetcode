// Time taken: 11:03, 11:06 -> Acc
struct Solution {}

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let total_sum = (n * (n + 1)) / 2;
        let mut num1 = 0;
        for val in 1..=n {
            if val % m != 0 {
                num1 += val;
            }
        }

        let num2 = total_sum - num1;
        return num1 - num2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
        assert_eq!(Solution::difference_of_sums(5, 1), -15);
    }
}
