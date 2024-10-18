// Time taken: 22:54, 22:58 -> Acc
struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = s.len() - 1;
        let mut count = (0, 0);
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];

        while left < right {
            if vowels.contains(&s[left].to_ascii_lowercase()) {
                count.0 += 1;
            }
            if vowels.contains(&s[right].to_ascii_lowercase()) {
                count.1 += 1;
            }
            left += 1;
            right -= 1;
        }

        return count.0 == count.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::halves_are_alike("book".to_string()), true);
        assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
    }
}
