// Time taken: 00:24, 00:30 -> Acc, 00:35 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const PRIME: u32 = 0b10100010100010101100;

        return (left..=right)
            .filter(|&num| {
                let count = num.count_ones();
                return (PRIME & (1 << count)) != 0;
            })
            .count() as i32;
    }

    // pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    //     let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    //     let mut result = 0;

    //     for num in left..=right {
    //         if primes.binary_search(&num.count_ones()).is_ok() {
    //             result += 1;
    //         }
    //     }

    //     return result;
    // }

    fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        if num == 2 {
            return true;
        }

        let range = num / 2;
        for n in 2..range {
            if num % n == 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}
