// Time taken: 13:24, 13:27, 14:31 - Optimized
struct Solution {}

impl Solution {
    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     let result: Vec<_> = haystack.match_indices(&needle).collect();

    //     if result.is_empty() {
    //         return -1;
    //     } else {
    //         return result[0].0 as i32;
    //     }
    // }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => {
                return -1;
            }
            Some(idx) => {
                return idx as i32;
            }
        }
    }

    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     let mut result: i32 = -1;

    //     if haystack.is_empty() || needle.is_empty() {
    //         return result;
    //     }

    //     let needle_vec: Vec<char> = needle.chars().collect();

    //     for (index, ch) in haystack.chars().enumerate() {
    //         if ch == needle_vec[0] {
    //             match haystack.get(index..) {
    //                 Some(sub_str) => {
    //                     if sub_str.starts_with(&needle) {
    //                         return index as i32;
    //                     }
    //                 }
    //                 None => {}
    //             }
    //         }
    //     }

    //     return result;
    // }

    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     let mut result: i32 = -1;

    //     let mut needle_index = 0;
    //     let mut start_found = false;
    //     let needle_vec: Vec<char> = needle.chars().collect();

    //     for (index, ch) in haystack.chars().enumerate() {
    //         if start_found {
    //             if ch != needle_vec[needle_index] {
    //                 start_found = false;
    //                 needle_index = 0;

    //                 if ch == needle_vec[needle_index] {
    //                     start_found = true;
    //                     needle_index += 1;

    //                     if needle_index == needle_vec.len() {
    //                         result = (index + 1 - needle_vec.len()) as i32;
    //                         break;
    //                     }
    //                 }
    //             } else {
    //                 needle_index += 1;

    //                 if needle_index == needle_vec.len() {
    //                     result = (index + 1 - needle_vec.len()) as i32;
    //                     break;
    //                 }
    //             }
    //         } else {
    //             if ch == needle_vec[needle_index] {
    //                 start_found = true;
    //                 needle_index += 1;
    //             }
    //         }
    //         println!(
    //             "Debug: {}, {}, {}, {}",
    //             index, ch, start_found, needle_index
    //         );
    //     }

    //     // if start_found && result == -1 && needle_index == needle_vec.len() {
    //     //     result = (haystack.len() - needle_vec.len()) as i32;
    //     // }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
        assert_eq!(
            Solution::str_str("butsad".to_string(), "sad".to_string()),
            3
        );
        assert_eq!(
            Solution::str_str("mississippi".to_string(), "issip".to_string()),
            4
        );
    }
}
