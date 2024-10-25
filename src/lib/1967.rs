// Time taken: 20:57, 21:05 -> Acc
struct Solution {}

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let word = word.chars().collect::<Vec<char>>();
        let mut result = 0;

        for pattern in patterns {
            let pattern = pattern.chars().collect::<Vec<char>>();
            let lps = Self::lps(&pattern);
            if Self::is_substring(&word, &pattern, &lps) {
                result += 1;
            }
        }

        return result;
    }

    pub fn lps(arr: &Vec<char>) -> Vec<usize> {
        let mut lps = vec![0; arr.len()];
        let mut left = 0;
        let mut idx = 1;

        while idx < arr.len() {
            if arr[idx] == arr[left] {
                left += 1;
                lps[idx] = left;
                idx += 1;
            } else {
                if left > 0 {
                    left = lps[left - 1];
                } else {
                    left = 0;
                    idx += 1;
                }
            }
        }

        return lps;
    }

    pub fn is_substring(arr: &Vec<char>, pattern: &Vec<char>, lps: &Vec<usize>) -> bool {
        let mut idx1 = 0;
        let mut idx2 = 0;
        let len1 = arr.len();
        let len2 = pattern.len();

        while (len1 - idx1) >= (len2 - idx2) {
            if arr[idx1] == pattern[idx2] {
                idx1 += 1;
                idx2 += 1;
                if idx2 == len2 {
                    return true;
                }
            } else {
                if idx2 > 0 {
                    idx2 = lps[idx2 - 1];
                } else {
                    idx2 = 0;
                    idx1 += 1;
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
            Solution::num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aaaaabbbbb".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string()],
                "ab".to_string()
            ),
            3
        );
    }
}
