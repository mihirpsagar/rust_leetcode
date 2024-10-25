// Time taken: 21:24, 21:26 -> Acc
struct Solution {}

impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut count = 2;
        let mut val = 2;
        let threshold = n / 2;

        while val <= threshold {
            if n % val == 0 {
                count += 1;
                if count > 3 {
                    break;
                }
            }
            val += 1;
        }

        return count == 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_three(2), false);
        assert_eq!(Solution::is_three(4), true);
    }
}
