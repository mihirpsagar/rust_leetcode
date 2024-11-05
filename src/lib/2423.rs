use std::collections::HashMap;

// Time taken: 00:01, 00:05, 00:09 -> Wrong, 00:20 -> Wrong, 00:43 -> Acc, 00:55 -> Optimized
struct Solution {}

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut arr = vec![0; 26];
        let a_ascii = 'a' as u8;

        for ch in word.chars() {
            arr[(ch as u8 - a_ascii) as usize] += 1;
        }

        for idx in 0..26 {
            if arr[idx] != 0 {
                if Self::is_valid(&arr, idx) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn is_valid(arr: &Vec<i32>, dec_idx: usize) -> bool {
        let mut prev = None;
        let mut idx = 0;

        while idx < arr.len() {
            if arr[idx] == 0 {
                idx += 1;
                continue;
            }

            let mut val = arr[idx];

            if idx == dec_idx {
                if val == 1 {
                    idx += 1;
                    continue;
                }
                val -= 1;
            }

            if let Some(prev_val) = prev {
                if val != prev_val {
                    return false;
                }
            } else {
                prev = Some(val);
            }

            idx += 1;
        }

        return true;
    }

    // pub fn equal_frequency(word: String) -> bool {
    //     let mut idx = 0;
    //     let word = word.chars().collect::<Vec<char>>();

    //     while idx < word.len() {
    //         if Self::is_valid(&word, idx) {
    //             return true;
    //         }
    //         idx += 1;
    //     }

    //     return false;
    // }

    // pub fn is_valid(arr: &Vec<char>, skip_idx: usize) -> bool {
    //     let mut map = HashMap::new();
    //     let mut prev = None;
    //     let mut idx = 0;

    //     while idx < arr.len() {
    //         if idx != skip_idx {
    //             *map.entry(arr[idx]).or_insert(0) += 1;
    //         }
    //         idx += 1;
    //     }

    //     for &val in map.values() {
    //         if let Some(prev_val) = prev {
    //             if val != prev_val {
    //                 return false;
    //             }
    //         } else {
    //             prev = Some(val);
    //         }
    //     }

    //     return true;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::equal_frequency("abcc".to_string()), true);
        assert_eq!(Solution::equal_frequency("aazz".to_string()), false);
        assert_eq!(Solution::equal_frequency("bac".to_string()), true);
        assert_eq!(Solution::equal_frequency("abbcc".to_string()), true);
    }
}
