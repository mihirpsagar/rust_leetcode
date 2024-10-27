// Time taken: 13:36, 13:41 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut word = word.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = word.len();
        let mut idx = 0;

        while idx < word.len() {
            if word[idx] == ch {
                right = idx;
                break;
            }
            idx += 1;
        }

        if right < word.len() {
            while left < right {
                let tmp = word[left];
                word[left] = word[right];
                word[right] = tmp;
                left += 1;
                right -= 1;
            }
        }

        return word.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd"
        );
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe"
        );
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }
}
