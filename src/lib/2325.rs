use std::collections::HashMap;

// Time taken: 13:38, 13:43 -> Acc
struct Solution {}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = HashMap::new();
        let mut result = Vec::new();
        let mut curr = 'a' as u8;

        for ch in key.chars() {
            if ch.is_ascii_alphabetic() {
                if !map.contains_key(&ch) {
                    map.insert(ch, curr as char);
                    curr += 1;
                }
            }
        }

        for ch in message.chars() {
            if let Some(&val) = map.get(&ch) {
                result.push(val);
            } else {
                result.push(ch);
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret"
        );
        assert_eq!(
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly"
        );
    }
}
