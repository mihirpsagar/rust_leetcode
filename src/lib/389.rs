// Time taken: 11:40, 11:44 -> Acc
struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut alpha = vec![0; 26];
        let a = b'a' as usize;

        for ch in t.chars() {
            alpha[ch as usize - a] += 1;
        }

        for ch in s.chars() {
            alpha[ch as usize - a] -= 1;
        }

        for (idx, &num) in alpha.iter().enumerate() {
            if num == 1 {
                return (idx + a) as u8 as char;
            }
        }

        return '_';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_the_difference(String::from("abcd"), String::from("abcde")),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference(String::from(""), String::from("y")),
            'y'
        );
    }
}
