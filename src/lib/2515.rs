// Time taken: 23:11, 23:17 -> Acc
struct Solution {}

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as usize;
        let len = words.len();

        if words[start_index] == target {
            return 0;
        }

        let mut left = 0;
        if start_index == 0 {
            left = len - 1;
        } else {
            left = start_index - 1;
        }
        let mut right = (start_index + 1) % len;
        let mut count = 1;
        loop {
            if words[left] == target || words[right] == target {
                return count;
            }
            count += 1;
            if left == right {
                break;
            }

            if left == 0 {
                left = len - 1;
            } else {
                left -= 1;
            }
            right = (right + 1) % len;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::closet_target(
                vec![
                    "hello".to_string(),
                    "i".to_string(),
                    "am".to_string(),
                    "leetcode".to_string(),
                    "hello".to_string()
                ],
                "hello".to_string(),
                1
            ),
            1
        );
        assert_eq!(
            Solution::closet_target(
                vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
                "leetcode".to_string(),
                0
            ),
            1
        );
        assert_eq!(
            Solution::closet_target(
                vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
                "ate".to_string(),
                0
            ),
            -1
        );
    }
}
