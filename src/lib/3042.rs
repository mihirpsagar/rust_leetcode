// Time taken: 23:42, 23:49 -> Acc
struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < words.len() - 1 {
            let mut idx2 = idx1 + 1;

            while idx2 < words.len() {
                if Self::is_prefix_and_suffix(&words[idx1], &words[idx2]) {
                    result += 1;
                }

                idx2 += 1;
            }
            idx1 += 1;
        }

        return result;
    }

    pub fn is_prefix_and_suffix(str1: &String, str2: &String) -> bool {
        let str1 = str1.chars().collect::<Vec<char>>();
        let str2 = str2.chars().collect::<Vec<char>>();

        let mut left = 0;
        let mut right = 0;
        while left < str1.len() && right < str2.len() {
            if str1[left] != str2[right] {
                break;
            }
            left += 1;
            right += 1;
        }

        if left != str1.len() {
            return false;
        }

        left = str1.len() - 1;
        right = str2.len() - 1;

        loop {
            if str1[left] != str2[right] {
                break;
            }

            if left == 0 || right == 0 {
                break;
            }
            left -= 1;
            right -= 1;
        }

        if left != 0 || str1[left] != str2[right] {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "a".to_string(),
                "aba".to_string(),
                "ababa".to_string(),
                "aa".to_string()
            ]),
            4
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "pa".to_string(),
                "papa".to_string(),
                "ma".to_string(),
                "mama".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string()]),
            0
        );
    }
}
