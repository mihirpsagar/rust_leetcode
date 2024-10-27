// Time taken: 11:40, 11:48 -> Acc, 12:10 -> Acc
struct Solution {}

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut prev_ch = 'a' as u8;
        let a_ascii = 'a' as u8;
        let z_ascii = 'z' as u8;
        let mut result = word.len() as i32;

        for ch in word.chars() {
            let ch = ch as u8;
            if ch > prev_ch {
                let diff = std::cmp::min(ch - prev_ch, (prev_ch - a_ascii) + (z_ascii - ch) + 1);
                result += diff as i32;
            } else {
                let diff = std::cmp::min(prev_ch - ch, (z_ascii - prev_ch) + (ch - a_ascii) + 1);
                result += diff as i32;
            }
            prev_ch = ch;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_time_to_type("abc".to_string()), 5);
        assert_eq!(Solution::min_time_to_type("bza".to_string()), 7);
        assert_eq!(Solution::min_time_to_type("zjpc".to_string()), 34);
    }
}
