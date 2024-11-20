// Time taken: 17:56, 18:04, 18:07 -> Acc, 13:02 -> Optimized
struct Solution {}

impl Solution {
    // Expand from center -> 0 ms
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut result = (0, 1); //(start, len)

        while idx < s.len() {
            //odd length palindrome
            Self::expand_from_center(&s, idx, idx, &mut result);

            //even length palindrome
            if idx + 1 < s.len() {
                Self::expand_from_center(&s, idx, idx + 1, &mut result);
            }

            idx += 1;
        }

        let mut result_str = Vec::new();
        idx = result.0;
        while idx < result.0 + result.1 {
            result_str.push(s[idx]);
            idx += 1;
        }

        return result_str.iter().collect();
    }

    pub fn expand_from_center(
        arr: &Vec<char>,
        mut left: usize,
        mut right: usize,
        result: &mut (usize, usize),
    ) {
        while arr[left] == arr[right] {
            if (right - left + 1) > result.1 {
                *result = (left, right - left + 1);
            }
            if left == 0 {
                break;
            }
            if right == arr.len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }

    // First try -> 42 ms
    // pub fn longest_palindrome(s: String) -> String {
    //     let s = s.chars().collect::<Vec<char>>();
    //     let mut result = (0, 1); //(start, len)
    //     let mut left = 0;

    //     while left < s.len() - 1 {
    //         let mut right = s.len() - 1;
    //         loop {
    //             if s[left] == s[right] {
    //                 if Self::is_palindrome(&s, left, right) {
    //                     if (right - left + 1) > result.1 {
    //                         result = (left, right - left + 1);
    //                     }
    //                     break;
    //                 }
    //             }
    //             if right - 1 == left {
    //                 break;
    //             }
    //             right -= 1;
    //         }

    //         left += 1;
    //     }

    //     let mut result_vec = Vec::new();
    //     let mut idx = result.0;
    //     while idx < result.0 + result.1 {
    //         result_vec.push(s[idx]);
    //         idx += 1;
    //     }

    //     return result_vec.iter().collect();
    // }

    // pub fn is_palindrome(arr: &Vec<char>, mut left: usize, mut right: usize) -> bool {
    //     while left < right {
    //         if arr[left] != arr[right] {
    //             return false;
    //         }
    //         left += 1;
    //         right -= 1;
    //     }

    //     return true;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
