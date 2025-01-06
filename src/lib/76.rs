// Time taken: 16:14, 16:30, 16:42 -> Wrong, 16:49 -> Acc, 17:34 -> Optimized

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map1 = HashMap::new();
        for ch in t.chars() {
            *map1.entry(ch).or_insert(0) += 1;
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut map2 = HashMap::new();
        let mut left = 0;
        let mut right = 0;
        let need = map1.len();
        let mut have = 0;
        let mut result = (s.len(), usize::MAX);

        while right < s.len() {
            *map2.entry(s[right]).or_insert(0) += 1;

            if map1.contains_key(&s[right])
                && *map2.get(&s[right]).unwrap() == *map1.get(&s[right]).unwrap()
            {
                have += 1;

                if have == need {
                    'inner: loop {
                        let val = map2.get(&s[left]).unwrap().clone();
                        if val == 1 {
                            map2.remove(&s[left]);
                        } else {
                            map2.insert(s[left], val - 1);
                        }
                        left += 1;

                        if let Some(val1) = map1.get(&s[left - 1]) {
                            if *val1 > (val - 1) {
                                have -= 1;
                                break 'inner;
                            }
                        }
                    }

                    let len = right - (left - 1) + 1;
                    if len < (result.1 - result.0 + 1) {
                        result = (left - 1, right);
                    }
                }
            }

            right += 1;
        }

        let mut result_str = Vec::new();

        if result.0 < s.len() {
            let mut idx = result.0;
            while idx <= result.1 {
                result_str.push(s[idx]);
                idx += 1;
            }
        }

        return result_str.iter().collect();
    }

    // pub fn min_window(s: String, t: String) -> String {
    //     let mut map1 = HashMap::new();
    //     for ch in t.chars() {
    //         *map1.entry(ch).or_insert(0) += 1;
    //     }

    //     let s = s.chars().collect::<Vec<char>>();
    //     let mut map2 = HashMap::new();
    //     let mut left = 0;
    //     let mut right = 0;
    //     let mut result = (s.len(), usize::MAX);
    //     while right < s.len() {
    //         *map2.entry(s[right]).or_insert(0) += 1;
    //         if Self::is_valid(&map2, &map1) {
    //             while Self::is_valid(&map2, &map1) {
    //                 let val = map2.get(&s[left]).unwrap();
    //                 if *val == 1 {
    //                     map2.remove(&s[left]);
    //                 } else {
    //                     map2.insert(s[left], *val - 1);
    //                 }
    //                 left += 1;
    //             }
    //             let len = right - (left - 1) + 1;
    //             if len < (result.1 - result.0 + 1) {
    //                 result = (left - 1, right);
    //             }
    //         }
    //         right += 1;
    //     }

    //     let mut result_str = Vec::new();

    //     if result.0 < s.len() {
    //         let mut idx = result.0;
    //         while idx <= result.1 {
    //             result_str.push(s[idx]);
    //             idx += 1;
    //         }
    //     }

    //     return result_str.iter().collect();
    // }

    // pub fn is_valid(map2: &HashMap<char, i32>, map1: &HashMap<char, i32>) -> bool {
    //     for (key, val) in map1 {
    //         if let Some(val2) = map2.get(key) {
    //             if *val > *val2 {
    //                 return false;
    //             }
    //         } else {
    //             return false;
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
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
        assert_eq!(
            Solution::min_window("bdab".to_string(), "ab".to_string()),
            "ab"
        );
    }
}
