// Time taken: 23:19, 23:22 -> Acc, 23:43 -> Optimized
struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let primes = [2, 3, 5, 7, 13];
        for prime in primes {
            // 2^(x-1) * 2^x - 1
            let left: u32 = 1 << (prime - 1);
            let right: u32 = (1 << prime) - 1;
            if let Some(val) = left.checked_mul(right) {
                if val == num as u32 {
                    return true;
                }
            }
        }
        return false;
    }

    // pub fn check_perfect_number(num: i32) -> bool {
    //     let mut sum = 0;
    //     let right = (num / 2) + 1;

    //     for val in 1..right {
    //         if num % val == 0 {
    //             sum += val;
    //             if sum > num {
    //                 return false;
    //             }
    //         }
    //     }

    //     return sum == num;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(7), false);
        assert_eq!(Solution::check_perfect_number(2096128), false);
    }
}
