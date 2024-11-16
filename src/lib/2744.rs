// Time taken: 11:54, 12:13 -> Wrong, 12:20 -> Acc

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut map: HashMap<[i32; 26], Vec<usize>> = HashMap::new();
        let mut result = 0;
        let mut idx = 0;
        let a_ascii = 'a' as u8;

        while idx < words.len() {
            let mut arr = [0; 26];
            for ch in words[idx].chars() {
                arr[(ch as u8 - a_ascii) as usize] += 1;
            }

            if let Some(val) = map.get_mut(&arr) {
                val.push(idx);
            } else {
                map.insert(arr, vec![idx]);
            }

            idx += 1;
        }

        for val in map.values() {
            if val.len() > 1 {
                result += Self::get_max_pairs(val, &words);
            }
        }

        return result;
    }

    pub fn get_max_pairs(arr: &Vec<usize>, words: &Vec<String>) -> i32 {
        let len = arr.len();
        let mut result = 0;
        let mut found = vec![false; len];
        let mut idx1 = 0;
        while idx1 < len - 1 {
            if found[idx1] {
                idx1 += 1;
                continue;
            }

            let mut idx2 = idx1 + 1;
            while idx2 < len {
                if found[idx2] {
                    idx2 += 1;
                    continue;
                }

                if Self::is_pair(&words[arr[idx1]], &words[arr[idx2]]) {
                    found[idx1] = true;
                    found[idx2] = true;
                    result += 1;
                    break;
                }

                idx2 += 1;
            }

            idx1 += 1;
        }

        return result;
    }

    fn is_pair(word1: &String, word2: &String) -> bool {
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        let mut idx1 = 0;
        let mut idx2 = word2.len() - 1;

        while idx1 < word1.len() {
            if word1[idx1] != word2[idx2] {
                return false;
            }
            idx1 += 1;
            if idx2 == 0 {
                break;
            }
            idx2 -= 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "cd".to_string(),
                "ac".to_string(),
                "dc".to_string(),
                "ca".to_string(),
                "zz".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "ab".to_string(),
                "ba".to_string(),
                "cc".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec!["aa".to_string(), "ab".to_string()]),
            0
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "ku".to_string(),
                "dd".to_string(),
                "gu".to_string(),
                "uk".to_string()
            ]),
            1
        );
    }
}
