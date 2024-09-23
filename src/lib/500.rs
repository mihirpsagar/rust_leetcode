use std::cmp::Ordering;

// Time taken: 20:06, 20:25 -> Acc
struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut row1: Vec<char> = "qwertyuiop".chars().collect();
        let mut row2: Vec<char> = "asdfghjkl".chars().collect();
        let mut row3: Vec<char> = "zxcvbnm".chars().collect();
        row1.sort();
        row2.sort();
        row3.sort();
        let rows = vec![row1, row2, row3];
        let mut result = Vec::new();

        'outer: for word in words {
            let letters: Vec<char> = word.chars().collect();
            let mut one_ch_found = false;
            'inner1: for row in &rows {
                for ch in &letters {
                    if Self::binary_search(&ch.to_ascii_lowercase(), &row) {
                        one_ch_found = true;
                    } else {
                        if one_ch_found {
                            continue 'outer;
                        }
                        continue 'inner1;
                    }
                }
                if one_ch_found {
                    result.push(word.clone());
                }
            }
        }

        return result;
    }

    fn binary_search(ch: &char, word: &Vec<char>) -> bool {
        let mut left = 0;
        let mut right = word.len();

        while left < right {
            let mid = left + ((right - left) / 2);
            match ch.cmp(&word[mid]) {
                Ordering::Equal => {
                    return true;
                }
                Ordering::Greater => {
                    left = mid + 1;
                }
                Ordering::Less => {
                    right = mid;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ]),
            vec!["Alaska", "Dad"]
        );
        assert_eq!(
            Solution::find_words(vec!["omk".to_string()]),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::find_words(vec!["adsdf".to_string(), "sfd".to_string()]),
            vec!["adsdf", "sfd"]
        );
    }
}
