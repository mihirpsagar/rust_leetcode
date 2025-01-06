// Time taken: 22:58, 23:07 -> Acc, 23:24 -> Optimized
struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut arr = Vec::new();
        for ch in s.chars() {
            if ch.is_alphanumeric() {
                arr.push(ch.to_ascii_lowercase());
            }
        }

        if arr.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = arr.len() - 1;
        while left < right {
            if arr[left] != arr[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        return true;
    }

    // pub fn is_palindrome(s: String) -> bool {
    //     let parsed_string: Vec<_> = s
    //         .chars()
    //         .filter_map(|c| c.is_alphanumeric().then(|| c.to_ascii_lowercase()))
    //         .collect();

    //     return parsed_string.iter().eq(parsed_string.iter().rev());
    // }

    // pub fn is_palindrome(s: String) -> bool {
    //     let s_bytes = s.as_bytes();
    //     let mut begin = 0;
    //     let mut end = s.len() - 1;

    //     loop {
    //         while begin < end {
    //             let ch = s_bytes[begin] as char;
    //             if ch.is_alphanumeric() {
    //                 break;
    //             }
    //             begin += 1;
    //         }

    //         while end > begin {
    //             let ch = s_bytes[end] as char;
    //             if ch.is_alphanumeric() {
    //                 break;
    //             }
    //             end -= 1;
    //         }

    //         if begin >= end {
    //             return true;
    //         }

    //         if (s_bytes[begin] as char).to_ascii_lowercase()
    //             != (s_bytes[end] as char).to_ascii_lowercase()
    //         {
    //             return false;
    //         }

    //         begin += 1;
    //         end -= 1;
    //     }
    // }

    // pub fn is_palindrome(s: String) -> bool {
    //     let parsed_string: Vec<char> = s
    //         .chars()
    //         .filter(|x| x.is_alphanumeric())
    //         .map(|x| x.to_ascii_lowercase())
    //         .collect();
    //     let mut result = true;

    //     for i in 0..parsed_string.len() / 2 {
    //         if parsed_string[i] != parsed_string[parsed_string.len() - i - 1] {
    //             result = false;
    //             break;
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
    }
}
