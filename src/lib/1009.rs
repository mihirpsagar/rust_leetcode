// Time taken: 00:11, 00:16 -> Acc
struct Solution {}

impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        let mut bits = Vec::new();

        while n > 0 {
            let bit = n % 2;
            if bit == 0 {
                bits.push(1);
            } else {
                bits.push(0);
            }
            n /= 2;
        }

        if bits.len() == 0 {
            return 1;
        }

        let mut result = 0;
        let mut idx = 0;
        while idx < bits.len() {
            if bits[idx] == 1 {
                result += 2_i32.pow(idx as u32);
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(7), 0);
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}
