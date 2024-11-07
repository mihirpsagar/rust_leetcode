// Time taken: 22:37, 22:42 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut result = 0;
        for str in strs {
            let len = Self::get_len(str);
            if len > result {
                result = len;
            }
        }

        return result;
    }

    pub fn get_len(word: String) -> i32 {
        for ch in word.chars() {
            if ch.is_ascii_alphabetic() {
                return word.len() as i32;
            }
        }

        return i32::from_str_radix(&word, 10).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]),
            5
        );
        assert_eq!(
            Solution::maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ]),
            1
        );
    }
}
