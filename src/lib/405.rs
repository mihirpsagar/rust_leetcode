// Time taken: 12:38, 13:04, 13:17 -> Acc
struct Solution {}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut bits = vec![0; 32];

        if num == 0 {
            return String::from("0");
        }

        if num < 0 {
            bits[0] = 1;
            let mut temp = i32::MIN;
            let mut idx = 1;
            while temp != num && idx < 32 {
                let pow = 2_i32.pow(31 - idx);
                if temp + pow <= num {
                    bits[idx as usize] = 1;
                    temp += pow;
                }
                idx += 1;
            }
        } else {
            let mut temp = num;
            let mut idx = 1;
            while temp != 0 && idx < 32 {
                let pow = 2_i32.pow(31 - idx);
                if temp - pow >= 0 {
                    bits[idx as usize] = 1;
                    temp -= pow;
                }
                idx += 1;
            }
        }

        let mut result = String::new();
        let a = b'a' as u8;
        for offset in 0..8 {
            let sum = 8 * bits[4 * offset + 0]
                + 4 * bits[4 * offset + 1]
                + 2 * bits[4 * offset + 2]
                + bits[4 * offset + 3];

            if sum == 0 && result == String::new() {
                continue;
            }

            if sum >= 10 {
                result += &((sum + a - 10) as char).to_string();
            } else {
                result += &sum.to_string();
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
        assert_eq!(Solution::to_hex(0), String::from("0"));
        assert_eq!(Solution::to_hex(26), String::from("1a"));
        assert_eq!(Solution::to_hex(-1), String::from("ffffffff"));
        assert_eq!(Solution::to_hex(111111), String::from("1b207"));
    }
}
