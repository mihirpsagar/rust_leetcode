// Time taken: 13:01, 13:13, 13:19 -> Acc
struct Solution {}

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut left = 0;
        let mut right = 0;

        while right < s.len() {
            if s[right] == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }

            if !Self::is_valid(zero_count, one_count, k) {
                while !Self::is_valid(zero_count, one_count, k) {
                    if s[left] == '0' {
                        zero_count -= 1;
                    } else {
                        one_count -= 1;
                    }
                    left += 1;
                }
            }
            result = result + (right - left + 1);

            right += 1;
        }

        return result as i32;
    }

    pub fn is_valid(x: i32, y: i32, k: i32) -> bool {
        return x <= k || y <= k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_k_constraint_substrings("10101".to_string(), 1),
            12
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("1010101".to_string(), 2),
            25
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("11111".to_string(), 1),
            15
        );
    }
}
