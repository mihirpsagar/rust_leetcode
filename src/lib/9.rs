// Time taken: 20:18, 20:26, 20:33 with optimization

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverse: i32 = 0;
        let mut num = x;

        while num > 0 {
            reverse = reverse * 10 + (num % 10);
            num /= 10;
        }

        return x == reverse;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
