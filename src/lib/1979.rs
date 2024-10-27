// Time taken: 12:14, 12:18 -> Acc
struct Solution {}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for num in nums {
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }

        return Self::gcd(max, min);
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Self::gcd(b, a % b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }
}
