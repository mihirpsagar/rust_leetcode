// Time taken: 17:14, 17:26 -> Acc
struct Solution {}

impl Solution {
    // pub fn is_power_of_three(n: i32) -> bool {
    //     let three_pow_19 = 1162261467;
    //     return n > 0 && three_pow_19 % n == 0;
    // }

    // pub fn is_power_of_three(n: i32) -> bool {
    //     let mut result = true;

    //     if n < 1 {
    //         return false;
    //     }

    //     let mut n = n;

    //     while n > 1 {
    //         if n % 3 == 0 {
    //             n /= 3;
    //         } else {
    //             return false;
    //         }
    //     }

    //     return result;
    // }

    pub fn is_power_of_three(n: i32) -> bool {
        let max_pow = (31f32 / 3f32.log2()) as u32;
        return n > 0 && 3_i32.pow(max_pow) % n == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
