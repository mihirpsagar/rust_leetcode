// Time taken: 22:34, 22:38 -> Acc
struct Solution {}

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut result = 0;

        for val in 2..=num {
            let mut val = val;
            let mut sum = 0;
            while val > 0 {
                sum = sum + (val % 10);
                val /= 10;
            }
            if sum % 2 == 0 {
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
        assert_eq!(Solution::count_even(4), 2);
        assert_eq!(Solution::count_even(30), 14);
    }
}
