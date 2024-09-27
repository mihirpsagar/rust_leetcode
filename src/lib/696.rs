// Time taken: 00:38, 00:48 -> Acc
struct Solution {}

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut prev_group_count = None;
        let mut curr_group_count = 1;
        let s = s.chars().collect::<Vec<char>>();

        for idx in 1..s.len() {
            if s[idx] != s[idx - 1] {
                result += 1;
                prev_group_count = Some(curr_group_count);
                curr_group_count = 1;
            } else {
                curr_group_count += 1;
                if let Some(prev_group_count) = prev_group_count {
                    if curr_group_count <= prev_group_count {
                        result += 1;
                    }
                }
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
        assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
        assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    }
}
