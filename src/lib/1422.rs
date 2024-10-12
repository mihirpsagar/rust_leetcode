// Time taken: 12:48, 12:52, 12:55 -> Acc
struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut num_zeros = 0;
        let mut num_ones = 0;

        for &ch in s.iter() {
            if ch == '1' {
                num_ones += 1;
            }
        }

        let mut idx = 0;
        while idx < s.len() - 1 {
            if s[idx] == '0' {
                num_zeros += 1;
            } else {
                num_ones -= 1;
            }

            if (num_zeros + num_ones) > result {
                result = num_zeros + num_ones
            }

            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
        assert_eq!(Solution::max_score("00111".to_string()), 5);
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }
}
