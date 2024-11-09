// Time taken: 10:36, 10:40 -> Wrong, 10:44 -> Acc
struct Solution {}

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut result = vec![0, 0];
        let mut arr = Vec::new();
        let mut n = n;
        while n > 0 {
            arr.push(n % 2);
            n /= 2;
        }

        let mut odd = false;
        for bit in arr {
            if bit == 1 {
                if odd {
                    result[1] += 1;
                } else {
                    result[0] += 1;
                }
            }
            odd = !odd;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::even_odd_bit(50), [1, 2]);
        assert_eq!(Solution::even_odd_bit(2), [0, 1]);
        assert_eq!(Solution::even_odd_bit(5), [2, 0]);
    }
}
