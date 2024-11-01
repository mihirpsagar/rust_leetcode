// Time taken: 21:59, 22:03, 22:08 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut num = num;

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        digits.sort();

        return std::cmp::min(
            std::cmp::min(
                Self::sum(&digits, (0, 1), (2, 3)),
                Self::sum(&digits, (0, 2), (1, 3)),
            ),
            Self::sum(&digits, (0, 3), (1, 2)),
        );
    }

    pub fn sum(arr: &Vec<i32>, d1: (usize, usize), d2: (usize, usize)) -> i32 {
        return ((arr[d1.0] * 10) + arr[d1.1]) + ((arr[d2.0] * 10) + arr[d2.1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_sum(2932), 52);
        assert_eq!(Solution::minimum_sum(4009), 13);
    }
}
