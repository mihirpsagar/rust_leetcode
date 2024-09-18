// Time taken: 15:25, 16:01
struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut sum: Vec<u8> = Vec::new();
        let mut a_vec: Vec<u8> = a
            .chars()
            .rev()
            .map(|x| if x == '0' { 0 } else { 1 })
            .collect();
        let mut b_vec: Vec<u8> = b
            .chars()
            .rev()
            .map(|x| if x == '0' { 0 } else { 1 })
            .collect();

        if b.len() < a.len() {
            let tmp = a_vec;
            a_vec = b_vec;
            b_vec = tmp;
        }

        let mut remainder: u8 = 0;
        let mut idx = 0;

        while idx < a_vec.len() {
            let val = a_vec[idx] + b_vec[idx] + remainder;
            sum.push(val % 2);
            remainder = val / 2;

            idx += 1;
        }

        while idx < b_vec.len() {
            let val = b_vec[idx] + remainder;
            sum.push(val % 2);
            remainder = val / 2;

            idx += 1;
        }

        if remainder == 1 {
            sum.push(remainder);
        }

        let result: Vec<&str> = sum
            .iter()
            .rev()
            .map(|&x| if x == 0 { "0" } else { "1" })
            .collect();

        return result.concat();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
    }
}
