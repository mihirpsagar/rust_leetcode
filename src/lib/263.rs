// Time taken: 15:26, 15:29 -> Acc
struct Solution {}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut result = true;

        if n < 1 {
            return false;
        }

        if n == 1 {
            return true;
        }

        let mut n = n;

        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
            } else if n % 3 == 0 {
                n /= 3;
            } else if n % 5 == 0 {
                n /= 5;
            } else {
                result = false;
                break;
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
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(14), false);
    }
}
