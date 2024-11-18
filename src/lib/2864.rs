// Time taken: 10:47, 10:50 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut one_count = 0;
        let len = s.len();
        let mut result = Vec::new();

        for ch in s.chars() {
            if ch == '1' {
                one_count += 1;
            }
        }

        for _ in 0..(one_count - 1) {
            result.push('1');
        }

        for _ in 0..(len - one_count) {
            result.push('0');
        }

        result.push('1');

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_string()),
            "001"
        );
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_string()),
            "1001"
        );
    }
}
