// Time taken: 16:22, 16:26 -> Acc
struct Solution {}

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let s3 = s3.chars().collect::<Vec<char>>();
        let mut idx = 0;

        while idx < s1.len() && idx < s2.len() && idx < s3.len() {
            if s1[idx] != s2[idx] || s1[idx] != s3[idx] {
                break;
            }
            idx += 1;
        }

        if idx == 0 {
            return -1;
        } else {
            return ((s1.len() - idx) + (s2.len() - idx) + (s3.len() - idx)) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_minimum_operations(
                "abc".to_string(),
                "abb".to_string(),
                "ab".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::find_minimum_operations(
                "dac".to_string(),
                "bac".to_string(),
                "cac".to_string()
            ),
            -1
        );
    }
}
