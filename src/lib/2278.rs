// Time taken: 02:47, 02:49 -> Acc
struct Solution {}

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let len = s.len();
        let mut count = 0;

        for ch in s.chars() {
            if ch == letter {
                count += 1;
            }
        }

        return ((count * 100) / len) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
