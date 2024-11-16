// Time taken: 21:10, 21:13 -> Acc
struct Solution {}

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut count = vec![0; 26];
        let a_ascii = 'a' as u8;
        let mut result = 0;

        for ch in s.chars() {
            count[(ch as u8 - a_ascii) as usize] += 1;
        }

        for num in count {
            if num > 0 {
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
        assert_eq!(Solution::minimized_string_length("aaabc".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("cbbd".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("baadccab".to_string()), 4);
    }
}
