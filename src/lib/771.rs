// Time taken: 00:44, 00:49 -> Acc
struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut lower = vec![false; 26];
        let mut upper = vec![false; 26];
        let lower_ascii = 'a' as u8;
        let upper_ascii = 'A' as u8;
        let mut result = 0;

        for ch in jewels.chars() {
            if ch.is_ascii_lowercase() {
                lower[(ch as u8 - lower_ascii) as usize] = true;
            } else {
                upper[(ch as u8 - upper_ascii) as usize] = true;
            }
        }

        for ch in stones.chars() {
            if ch.is_ascii_lowercase() {
                if lower[(ch as u8 - lower_ascii) as usize] {
                    result += 1;
                }
            } else {
                if upper[(ch as u8 - upper_ascii) as usize] {
                    result += 1;
                }
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
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
        assert_eq!(
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
            0
        );
    }
}
