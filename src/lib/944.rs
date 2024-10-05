// Time taken: 12:59, 13:04, 13:07 -> Wrong, 13:13 -> Acc
struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut result = vec![true; strs[0].len()];
        let mut prev: Vec<char> = Vec::new();

        for word in strs {
            let word: Vec<char> = word.chars().collect();
            if prev.is_empty() {
                prev = word.clone();
            } else {
                let mut idx = 0;
                while idx < word.len() {
                    if prev[idx] > word[idx] {
                        result[idx] = false;
                    }
                    prev[idx] = word[idx];
                    idx += 1;
                }
            }
        }

        return result.iter().filter(|&&x| x == false).count() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]),
            0
        );
        assert_eq!(
            Solution::min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::min_deletion_size(vec![
                "rrjk".to_string(),
                "furt".to_string(),
                "guzm".to_string()
            ]),
            2
        );
    }
}
