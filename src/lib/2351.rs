// Time taken: 22:01, 22:02 -> Acc
struct Solution {}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut arr = vec![0; 26];
        let a_ascii = 'a' as u8;

        for ch in s.chars() {
            if arr[(ch as u8 - a_ascii) as usize] == 1 {
                return ch;
            }
            arr[(ch as u8 - a_ascii) as usize] += 1;
        }

        return '_';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}
