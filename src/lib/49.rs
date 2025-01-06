// Time taken: 11:35, 11:42 -> Acc

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        let mut result = Vec::new();
        let a = 'a' as u8;

        for str in strs {
            let mut count = vec![0; 26];
            for ch in str.chars() {
                count[(ch as u8 - a) as usize] += 1;
            }
            if let Some(arr) = map.get_mut(&count) {
                arr.push(str.clone());
            } else {
                map.insert(count.clone(), vec![str.clone()]);
            }
        }

        for val in map.values() {
            result.push(val.clone());
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }
}
