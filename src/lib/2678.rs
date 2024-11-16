// Time taken: 12:07, 12:09 -> Acc
struct Solution {}

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut result = 0;
        for word in details {
            let val = u8::from_str_radix(&word[11..13], 10).unwrap();
            if val > 60 {
                result += 1;
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
        assert_eq!(
            Solution::count_seniors(vec![
                "7868190130M7522".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::count_seniors(vec![
                "1313579440F2036".to_string(),
                "2921522980M5644".to_string()
            ]),
            0
        );
    }
}
