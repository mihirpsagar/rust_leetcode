use std::collections::HashMap;

// Time taken: 12:40, 12:43, 12:46, 12:53 -> Acc
struct Solution {}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut result = None;

        for ch in s.chars() {
            *map1.entry(ch).or_insert(0) += 1;
        }

        for ch in target.chars() {
            *map2.entry(ch).or_insert(0) += 1;
        }

        for (key2, val2) in map2 {
            if let Some(&val1) = map1.get(&key2) {
                let res = val1 / val2;
                if let Some(prev) = result {
                    if res < prev {
                        result = Some(res);
                    }
                } else {
                    result = Some(res);
                }
            } else {
                return 0;
            }
        }

        if let Some(val) = result {
            return val;
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()),
            2
        );
        assert_eq!(
            Solution::rearrange_characters("abcba".to_string(), "abc".to_string()),
            1
        );
        assert_eq!(
            Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }
}
