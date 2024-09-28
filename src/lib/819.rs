// Time taken: 01:44, 01:55 -> Wrong, 01:59 -> Acc
struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map = std::collections::HashMap::new();
        let non_alpha = vec!["!", "?", "'", ",", ";", ".", " "];
        let mut paragraph = paragraph;

        for ch in non_alpha.iter() {
            paragraph = paragraph.replace(ch, "#");
        }

        let words = paragraph.split("#").filter(|x| !x.is_empty()).into_iter();
        let mut result = (String::new(), 0);

        for word in words {
            let word = word.to_ascii_lowercase();

            if !banned.contains(&word) {
                if let Some(val) = map.get(&word) {
                    map.insert(word, val + 1);
                } else {
                    map.insert(word, 1);
                }
            }
        }

        for (key, val) in map {
            if val > result.1 {
                result = (key, val);
            }
        }

        return result.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball"
        );
        assert_eq!(Solution::most_common_word("a.".to_string(), vec![]), "a");
        assert_eq!(
            Solution::most_common_word("a, a, a, a, b,b,b,c, c".to_string(), vec!["a".to_string()]),
            "b"
        );
    }
}
