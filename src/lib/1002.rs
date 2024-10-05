// Time taken: 23:46, 23:57 -> Acc
struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();

        for word in words {
            let mut word_vec = Vec::new();
            for ch in word.chars() {
                word_vec.push(ch.to_string());
            }

            if result.is_empty() {
                result = word_vec;
            } else {
                let mut common = Vec::new();
                let mut idx1 = 0;
                while idx1 < word_vec.len() {
                    let mut idx2 = 0;
                    while idx2 < result.len() {
                        if word_vec[idx1] == result[idx2] {
                            common.push(result[idx2].clone());
                            result.remove(idx2);
                            break;
                        }
                        idx2 += 1;
                    }
                    idx1 += 1;
                }
                if common.is_empty() {
                    return common;
                }
                result = common;
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
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            ["e", "l", "l"]
        );
        assert_eq!(
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            ["c", "o"]
        );
    }
}
