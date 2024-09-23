// Time taken: 19:55, 20:01, 20:05 -> Acc
struct Solution {}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s_vec: Vec<char> = s.chars().rev().collect();
        let mut result: Vec<char> = Vec::new();
        let mut idx = 0;
        let mut group_size = 0;

        while idx < s_vec.len() {
            if s_vec[idx] == '-' {
                idx += 1;
                continue;
            }

            if group_size == k {
                result.push('-');
                result.push(s_vec[idx].to_ascii_uppercase());
                group_size = 1;
            } else {
                result.push(s_vec[idx].to_ascii_uppercase());
                group_size += 1;
            }
            idx += 1;
        }

        result.reverse();

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::license_key_formatting(String::from("5F3Z-2e-9-w"), 4),
            "5F3Z-2E9W"
        );
        assert_eq!(
            Solution::license_key_formatting(String::from("2-5g-3-J"), 2),
            "2-5G-3J"
        );
    }
}
