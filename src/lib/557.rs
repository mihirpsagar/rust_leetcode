// Time taken: 19:03, 19:06 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        return s
            .split_ascii_whitespace()
            .map(|str| str.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
        assert_eq!(Solution::reverse_words("Mr Ding".to_string()), "rM gniD");
    }
}
