// Time taken: 10:48, 10:50 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count = vec![(0, 0); 26];
        let mut result = 0;
        let small_a = 'a' as u8;
        let capital_a = 'A' as u8;

        for ch in word.chars() {
            if ch.is_ascii_lowercase() {
                count[(ch as u8 - small_a) as usize].0 += 1;
            } else {
                count[(ch as u8 - capital_a) as usize].1 += 1;
            }
        }

        for val in count {
            if val.0 > 0 && val.1 > 0 {
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
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
    }
}
