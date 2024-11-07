// Time taken: 21:03, 21:06 -> Acc
struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut left_sum = 0;
        let mut right_sum = (n * (n + 1)) / 2;
        let mut idx = 1;

        while idx <= n {
            left_sum += idx;
            right_sum -= idx - 1;
            if left_sum == right_sum {
                return idx;
            }
            idx += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
