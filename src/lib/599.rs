use std::collections::HashMap;

// Time taken: 00:38, 00:47 -> Wrong, 00:56 -> Acc
struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut result: Option<(Vec<String>, usize)> = None;

        for (idx, word) in list1.iter().enumerate() {
            map.insert(word, idx);
        }

        for (idx2, word) in list2.iter().enumerate() {
            if let Some(idx) = map.get(word) {
                if let Some((prev_word_vec, prev_diff)) = result.as_mut() {
                    if (idx + idx2) < *prev_diff {
                        result = Some((vec![word.to_string()], idx + idx2));
                    } else if (idx + idx2) == *prev_diff {
                        prev_word_vec.push(word.to_string());
                    }
                } else {
                    result = Some((vec![word.to_string()], idx + idx2));
                }
            }
        }

        return result.unwrap().0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KNN".to_string(),
                    "KFC".to_string(),
                    "Burger King".to_string(),
                    "Tapioca Express".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec!["KFC", "Burger King", "Tapioca Express", "Shogun"]
        );
    }
}
