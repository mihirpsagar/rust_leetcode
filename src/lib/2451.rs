use std::collections::HashMap;

// Time taken: 03:14, 03:26 -> Acc
struct Solution {}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut map: HashMap<Vec<i8>, Vec<usize>> = HashMap::new();
        for (idx, word) in words.iter().enumerate() {
            let arr = Self::get_diff(word);
            if let Some(val) = map.get_mut(&arr) {
                val.push(idx);
            } else {
                map.insert(arr, vec![idx]);
            }

            if map.len() > 1 && idx > 1 {
                break;
            }
        }

        for val in map.values() {
            if val.len() == 1 {
                return words[val[0]].clone();
            }
        }

        return String::new();
    }

    pub fn get_diff(word: &String) -> Vec<i8> {
        let mut idx = 1;
        let a_ascii = 'a' as u8;
        let word = word.chars().collect::<Vec<char>>();
        let mut res = Vec::new();

        while idx < word.len() {
            res.push((word[idx] as u8 - a_ascii) as i8 - (word[idx - 1] as u8 - a_ascii) as i8);
            idx += 1;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::odd_string(vec![
                "adc".to_string(),
                "wzy".to_string(),
                "abc".to_string()
            ]),
            "abc"
        );
        assert_eq!(
            Solution::odd_string(vec![
                "aaa".to_string(),
                "bob".to_string(),
                "ccc".to_string(),
                "ddd".to_string()
            ]),
            "bob"
        );
    }
}
