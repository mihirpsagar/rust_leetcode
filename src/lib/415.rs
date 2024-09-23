// Time taken: 13:48, 14:02, 14:10 -> Acc
struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut result: Vec<char> = Vec::new();
        let num1_vec: Vec<char> = num1.chars().rev().collect();
        let num2_vec: Vec<char> = num2.chars().rev().collect();
        let ascii_0 = '0' as u8;

        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut remainder = 0;

        while idx1 < num1_vec.len() && idx2 < num2_vec.len() {
            let sum =
                (num1_vec[idx1] as u8 - ascii_0) + (num2_vec[idx2] as u8 - ascii_0) + remainder;
            result.push(((sum % 10) + ascii_0) as char);
            remainder = sum / 10;
            idx1 += 1;
            idx2 += 1;
        }

        while idx1 < num1_vec.len() {
            let sum = (num1_vec[idx1] as u8 - ascii_0) + remainder;
            result.push(((sum % 10) + ascii_0) as char);
            remainder = sum / 10;
            idx1 += 1;
        }

        while idx2 < num2_vec.len() {
            let sum = (num2_vec[idx2] as u8 - ascii_0) + remainder;
            result.push(((sum % 10) + ascii_0) as char);
            remainder = sum / 10;
            idx2 += 1;
        }

        if remainder != 0 {
            result.push((remainder + ascii_0) as char);
        }

        return result.iter().rev().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_strings(String::from("11"), String::from("123")),
            "134"
        );
        assert_eq!(
            Solution::add_strings(String::from("456"), String::from("77")),
            "533"
        );
        assert_eq!(
            Solution::add_strings(String::from("0"), String::from("0")),
            "0"
        );
    }
}
