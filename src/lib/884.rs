// Time taken: 23:48, 23:52 -> Acc
struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut map = std::collections::HashMap::new();

        for word in s1.split_ascii_whitespace() {
            if let Some(val) = map.get(word) {
                map.insert(word, val + 1);
            } else {
                map.insert(word, 1);
            }
        }

        for word in s2.split_ascii_whitespace() {
            if let Some(val) = map.get(word) {
                map.insert(word, val + 1);
            } else {
                map.insert(word, 1);
            }
        }

        for (key, val) in map {
            if val == 1 {
                result.push(key.to_string());
            }
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
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            )
            .contains(&"sweet".to_string()),
            true
        );
        assert_eq!(
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            )
            .contains(&"sour".to_string()),
            true
        );
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec!["banana"]
        );
    }
}
