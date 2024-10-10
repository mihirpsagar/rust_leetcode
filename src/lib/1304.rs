// Time taken: 23:33, 23:36 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        if n % 2 != 0 {
            result.push(0);
        }
        let n = n / 2;

        for num in 1..=n {
            result.push(num);
            result.push(-1 * num);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_zero(5), vec![0, 1, -1, 2, -2]);
        assert_eq!(Solution::sum_zero(3), vec![0, 1, -1]);
        assert_eq!(Solution::sum_zero(1), vec![0]);
    }
}
