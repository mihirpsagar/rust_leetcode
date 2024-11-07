use std::collections::HashMap;

// Time taken: 22:49, 22:54 -> Acc
struct Solution {}

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        let a_ascii = 'a' as u8;
        let mut map = HashMap::new();

        for word in words {
            let mut arr = vec![false; 26];
            for ch in word.chars() {
                arr[(ch as u8 - a_ascii) as usize] = true;
            }

            if let Some(&val) = map.get(&arr) {
                result += val;
            }

            *map.entry(arr).or_insert(0) += 1;
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
            Solution::similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()]),
            3
        );
        assert_eq!(
            Solution::similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ]),
            0
        );
    }
}
