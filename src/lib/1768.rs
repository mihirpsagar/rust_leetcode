// Time taken: 13:08, 13:12 -> Acc
struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        let mut result = Vec::new();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < word1.len() && idx2 < word2.len() {
            result.push(word1[idx1]);
            result.push(word2[idx2]);
            idx1 += 1;
            idx2 += 1;
        }

        while idx1 < word1.len() {
            result.push(word1[idx1]);
            idx1 += 1;
        }

        while idx2 < word2.len() {
            result.push(word2[idx2]);
            idx2 += 1;
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }
}
