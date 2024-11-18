use std::collections::HashMap;

// Time taken: 13:15, 13:39 -> Wrong, 13:59 -> Acc
struct Solution {}

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut prev_group = groups[0];
        result.push(words[0].clone());

        let mut idx = 1;
        while idx < words.len() {
            if groups[idx] != prev_group {
                prev_group = groups[idx];
                result.push(words[idx].clone());
            }

            idx += 1;
        }

        return result;
    }

    // pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    //     let mut result = Vec::new();
    //     let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    //     let mut idx = 0;

    //     while idx < groups.len() {
    //         if let Some(val) = map.get_mut(&groups[idx]) {
    //             val.push(idx);
    //         } else {
    //             map.insert(groups[idx], vec![idx]);
    //         }

    //         idx += 1;
    //     }

    //     let mut prev_key = -1;
    //     loop {
    //         if let Some(res) = Self::get_next_idx(&mut map, prev_key) {
    //             result.push(words[res.1].clone());
    //             prev_key = res.0;
    //         } else {
    //             break;
    //         }
    //     }

    //     return result;
    // }

    // pub fn get_next_idx(map: &mut HashMap<i32, Vec<usize>>, prev_key: i32) -> Option<(i32, usize)> {
    //     let mut res = (-1, 0);
    //     let mut max_len = 0;

    //     for (key, val) in map.iter_mut() {
    //         if *key == prev_key {
    //             continue;
    //         }

    //         if val.len() > max_len {
    //             res.0 = *key;
    //             max_len = val.len();
    //         }
    //     }

    //     if max_len == 0 {
    //         return None;
    //     }

    //     res.1 = map.get_mut(&res.0).unwrap().pop().unwrap();

    //     return Some(res);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_longest_subsequence(
                vec!["e".to_string(), "a".to_string(), "b".to_string()],
                vec![0, 0, 1]
            ),
            ["e", "b"]
        );
        assert_eq!(
            Solution::get_longest_subsequence(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                vec![1, 0, 1, 1]
            ),
            ["a", "b", "c"]
        );
    }
}
