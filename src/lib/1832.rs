// Time taken: 20:46, 20:48 -> Acc
struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut arr = vec![false; 26];
        let a_ascii = 'a' as u8;

        for ch in sentence.chars() {
            arr[(ch as u8 - a_ascii) as usize] = true;
        }

        for val in arr {
            if !val {
                return false;
            }
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
            Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
            true
        );
        assert_eq!(Solution::check_if_pangram("leetcode".to_string()), false);
    }
}
