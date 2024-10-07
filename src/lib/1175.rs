// Time taken: 01:18, 01:30 -> Acc
struct Solution {}

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const MOD: u128 = 1_000_000_007;
        let prime_count = Self::number_of_primes(n);
        let non_prime_count: u128 = n as u128 - prime_count;
        let mut result: u128 = 1;

        for n in 2..=prime_count {
            result = result * n % MOD;
        }

        for n in 2..=non_prime_count {
            result = result * n % MOD;
        }

        return result as i32;
    }

    fn number_of_primes(n: i32) -> u128 {
        let mut count = 0;

        'outer: for num in 2..=n {
            let max = num / 2;
            for num2 in 2..=max {
                if num % num2 == 0 {
                    continue 'outer;
                }
            }
            count += 1;
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_prime_arrangements(5), 12);
        assert_eq!(Solution::num_prime_arrangements(100), 682289015);
    }
}
