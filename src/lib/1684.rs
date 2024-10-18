use std::collections::HashSet;

// Time taken: 22:20, 22:23 -> Acc, 22:28 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut arr = vec![false; 26];
        let a_ascii = 'a' as u8;
        let mut result = 0;

        for ch in allowed.chars() {
            arr[(ch as u8 - a_ascii) as usize] = true;
        }

        for word in words {
            let mut is_valid = true;
            for ch in word.chars() {
                if !arr[(ch as u8 - a_ascii) as usize] {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                result += 1;
            }
        }

        return result;
    }
    // pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    //     let mut result = 0;
    //     let mut set = HashSet::new();

    //     for ch in allowed.chars() {
    //         set.insert(ch);
    //     }

    //     for word in words {
    //         let mut is_valid = true;
    //         for ch in word.chars() {
    //             if !set.contains(&ch) {
    //                 is_valid = false;
    //                 break;
    //             }
    //         }
    //         if is_valid {
    //             result += 1;
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
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            ),
            7
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            ),
            4
        );
    }
}
