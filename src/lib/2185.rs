// Time taken: 22:38, 22:41 -> Acc
struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut result = 0;
        let pref = pref.chars().collect::<Vec<char>>();

        for word in words {
            let arr = word.chars().collect::<Vec<char>>();
            let mut left = 0;
            let mut right = 0;
            while left < arr.len() && right < pref.len() {
                if arr[left] != pref[right] {
                    break;
                }
                left += 1;
                right += 1;
            }

            if right == pref.len() {
                result += 1;
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
            Solution::prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string()
                ],
                "code".to_string()
            ),
            0
        );
    }
}
