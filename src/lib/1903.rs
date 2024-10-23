// Time taken: 23:21, 23:29 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let num = num.chars().collect::<Vec<char>>();
        let mut idx = num.len() - 1;

        loop {
            if num[idx] == '1'
                || num[idx] == '3'
                || num[idx] == '5'
                || num[idx] == '7'
                || num[idx] == '9'
            {
                return num[0..(idx + 1)].iter().collect();
            }

            if idx == 0 {
                break;
            }

            idx -= 1;
        }

        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_odd_number("52".to_string()), "5");
        assert_eq!(Solution::largest_odd_number("4206".to_string()), "");
        assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427");
    }
}
