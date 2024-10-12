// Time taken: 10:46, 11:01, 11:08 -> Wrong, 11:09 -> Acc
struct Solution {}

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();

        let mut idx1 = 0;
        while idx1 < words.len() {
            let mut idx2 = 0;
            let mut lcs = Vec::new();
            while idx2 < words.len() {
                if words[idx1].len() > words[idx2].len() || idx1 == idx2 {
                    idx2 += 1;
                    continue;
                }

                if lcs.is_empty() {
                    lcs = Self::lcs(words[idx1].clone());
                }

                if Self::is_valid_substring(words[idx2].clone(), words[idx1].clone(), &lcs) {
                    result.push(words[idx1].clone());
                    break;
                }

                idx2 += 1;
            }

            idx1 += 1;
        }

        return result;
    }

    fn lcs(word: String) -> Vec<usize> {
        let word = word.chars().collect::<Vec<char>>();
        let mut lcs = vec![0; word.len()];

        let mut left = 0;
        // lcs[0] = 0;
        let mut idx = 1;

        while idx < word.len() {
            if word[left] == word[idx] {
                left += 1;
                lcs[idx] = left;
                idx += 1;
            } else {
                if left > 0 {
                    left = lcs[left - 1];
                } else {
                    left = 0;
                    // lcs[idx] = 0;
                    idx += 1;
                }
            }
        }

        return lcs;
    }

    fn is_valid_substring(source: String, sub: String, lcs: &Vec<usize>) -> bool {
        let source = source.chars().collect::<Vec<char>>();
        let sub = sub.chars().collect::<Vec<char>>();

        let m = source.len();
        let n = sub.len();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while (m - idx1) >= (n - idx2) {
            if source[idx1] == sub[idx2] {
                idx1 += 1;
                idx2 += 1;

                if idx2 == n {
                    return true;
                }
            } else {
                if idx2 > 0 {
                    idx2 = lcs[idx2 - 1];
                } else {
                    // idx2 = 0;
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
            Solution::string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string()
            ]),
            ["as", "hero"]
        );
        assert_eq!(
            Solution::string_matching(vec![
                "leetcode".to_string(),
                "et".to_string(),
                "code".to_string()
            ]),
            ["et", "code"]
        );
        assert!(Solution::string_matching(vec![
            "blue".to_string(),
            "green".to_string(),
            "bu".to_string()
        ])
        .is_empty());
        assert_eq!(
            Solution::string_matching(vec![
                "leetcoder".to_string(),
                "leetcode".to_string(),
                "od".to_string(),
                "hamlet".to_string(),
                "am".to_string()
            ]),
            ["leetcode", "od", "am"]
        );
    }
}
