// Time taken: 11:46, 11:52, 11:55 -> Acc
struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut p1 = 0;
        let mut p2 = 0;
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        while p1 < s.len() && p2 < t.len() {
            if s[p1] == t[p2] {
                p1 += 1;
            }
            p2 += 1;
        }

        return p1 == s.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
        assert_eq!(
            Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")),
            false
        );
    }
}
