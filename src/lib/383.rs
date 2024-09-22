use std::collections::HashMap;

// Time taken: 11:06, 11:11 -> Acc, 11:17 -> Better, 11:23 -> Optimized
struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut alpha = vec![0; 26];
        let mut a = b'a' as usize;

        for ch in magazine.chars() {
            alpha[ch as usize - a] += 1;
        }

        for ch in ransom_note.chars() {
            if alpha[ch as usize - a] == 0 {
                return false;
            } else {
                alpha[ch as usize - a] -= 1;
            }
        }

        return true;
    }

    // pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    //     let mut magazine = magazine;
    //     for ch in ransom_note.chars() {
    //         if let Some(_) = magazine.chars().position(|x| x == ch) {
    //             magazine = magazine.replacen(ch, "", 1);
    //         } else {
    //             return false;
    //         }
    //     }

    //     return true;
    // }
    // pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    //     let mut hash_map = HashMap::new();
    //     let mut result = true;

    //     for ch in magazine.chars() {
    //         match hash_map.get(&ch) {
    //             None => {
    //                 hash_map.insert(ch, 1);
    //             }
    //             Some(&val) => {
    //                 hash_map.insert(ch, val + 1);
    //             }
    //         }
    //     }

    //     for ch in ransom_note.chars() {
    //         match hash_map.get(&ch) {
    //             None => {
    //                 return false;
    //             }
    //             Some(&val) => {
    //                 if val == 0 {
    //                     return false;
    //                 } else {
    //                     hash_map.insert(ch, val - 1);
    //                 }
    //             }
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
            Solution::can_construct(String::from("a"), String::from("b")),
            false
        );
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("ab")),
            false
        );
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("aab")),
            true
        );
    }
}
