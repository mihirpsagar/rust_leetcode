// Time taken: 00:05, 00:15, 00:22 -> Acc
struct Solution {}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut alpha = vec![0; 26];
        let a_ascii = 'a' as u8;
        let mut min_word: Option<String> = None;

        for ch in license_plate.chars() {
            if ch.is_ascii_alphabetic() {
                alpha[(ch.to_ascii_lowercase() as u8 - a_ascii) as usize] += 1;
            }
        }

        for word in words {
            let mut found = alpha.clone();
            for ch in word.chars() {
                let pos = (ch.to_ascii_lowercase() as u8 - a_ascii) as usize;
                if found[pos] > 0 {
                    found[pos] -= 1;
                }
            }

            if found.iter().sum::<i32>() == 0 {
                if let Some(min_word_prev) = min_word.clone() {
                    if word.len() < min_word_prev.len() {
                        min_word = Some(word);
                    }
                } else {
                    min_word = Some(word);
                }
            }
        }

        return min_word.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            ),
            "steps"
        );
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            ),
            "pest"
        );
    }
}
