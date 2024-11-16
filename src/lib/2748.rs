// Time taken: 12:42, 12:52- > Acc

struct Solution {}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;
        while idx1 < nums.len() - 1 {
            let digit1 = Self::get_first_digit(nums[idx1]);

            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if Self::gcd(digit1, nums[idx2] % 10) == 1 {
                    result += 1;
                }

                idx2 += 1;
            }

            idx1 += 1;
        }

        return result;
    }

    pub fn get_first_digit(num: i32) -> i32 {
        let mut num = num;
        while num > 9 {
            num /= 10;
        }
        return num;
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        } else {
            return Self::gcd(b, a % b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
        assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
    }
}
