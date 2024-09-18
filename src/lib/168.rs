// Time taken: 00:34, 01:16
struct Solution {}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        let mut val = column_number;

        while val > 0 {
            val -= 1;
            let ch = ((val % 26) as u8) + b'A';
            result.insert(0, ch as char);
            val /= 26;
        }

        return result;
    }

    // pub fn convert_to_title(column_number: i32) -> String {
    //     let mut result = String::new();
    //     let mut val = column_number;

    //     for pow in (0..7).rev() {
    //         let num = 27_i32.pow(pow);
    //         if (val / num as i32) > 0 {
    //             let ch = ((val / num as i32) + 64) as u8 as char;
    //             result += &ch.to_string();
    //             val /= 27;
    //         }
    //     }

    //     return result;
    // }

    // pub fn convert_to_title(column_number: i32) -> String {
    //     let mut result = String::new();
    //     let mut val = column_number;

    //     loop {
    //         if val / 27 == 0 {
    //             let ch = ((val % 27) + 64) as u8 as char;
    //             result += &ch.to_string();
    //             break;
    //         }

    //         let ch = (((val / 27) % 27) + 64) as u8 as char;
    //         result += &ch.to_string();
    //         val /= 27;
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
