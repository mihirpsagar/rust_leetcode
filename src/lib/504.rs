// Time taken: 21:26, 21:42 -> Wrong, 21:44 -> Acc
struct Solution {}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }

        let mut result: Vec<char> = vec![];
        let is_neg = num < 0;
        let mut num = num.abs();
        let ascii_0 = '0' as u8;

        while num > 0 {
            let digit = num % 7;
            result.push((digit as u8 + ascii_0) as char);
            num /= 7;
        }

        result.reverse();

        let mut result: String = result.iter().collect();

        if is_neg {
            result.insert(0, '-');
        }

        return result;
    }
    // pub fn convert_to_base7(num: i32) -> String {
    //     let mut result: Vec<char> = Vec::new();
    //     let neg = if num < 0 { true } else { false };
    //     let mut num = num.abs();
    //     let ascii_0 = '0' as u8;
    //     let mut leading_zero = true;

    //     for pow in (0..9).rev() {
    //         let val = 7_i32.pow(pow);
    //         let digit = num / val;
    //         if digit == 0 && leading_zero {
    //             continue;
    //         } else {
    //             leading_zero = false;
    //             result.push((ascii_0 + digit as u8) as char);
    //             num = num - (digit * val);
    //         }
    //     }

    //     if result.is_empty() {
    //         result.push('0');
    //     }

    //     if neg {
    //         let str: String = result.iter().collect();
    //         return String::from("-") + &str;
    //     } else {
    //         return result.iter().collect();
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_to_base7(100), "202");
        assert_eq!(Solution::convert_to_base7(-7), "-10");
        assert_eq!(Solution::convert_to_base7(10000000), "150666343");
    }
}
