// Time taken: 13:53, 13:54 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut num = num.chars().collect::<Vec<char>>();
        let mut idx = num.len() - 1;
        while num[idx] == '0' {
            num.pop();
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return num.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_trailing_zeros("51230100".to_string()),
            "512301"
        );
        assert_eq!(Solution::remove_trailing_zeros("123".to_string()), "123");
    }
}
