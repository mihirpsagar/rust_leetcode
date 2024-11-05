// Time taken: 13:21, 13:25 -> Acc
struct Solution {}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut lower_arr = vec![false; 26];
        let mut upper_arr = vec![false; 26];
        let lower_a_ascii = 'a' as u8;
        let upper_a_ascii = 'A' as u8;

        for ch in s.chars() {
            if ch.is_ascii_lowercase() {
                lower_arr[(ch as u8 - lower_a_ascii) as usize] = true;
            } else {
                upper_arr[(ch as u8 - upper_a_ascii) as usize] = true;
            }
        }

        let mut idx = lower_arr.len() - 1;
        loop {
            if lower_arr[idx] && upper_arr[idx] {
                return ((idx as u8 + upper_a_ascii) as char).to_string();
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::greatest_letter("lEeTcOdE".to_string()), "E");
        assert_eq!(Solution::greatest_letter("arRAzFif".to_string()), "R");
        assert_eq!(Solution::greatest_letter("AbCdEfGhIjK".to_string()), "");
    }
}
