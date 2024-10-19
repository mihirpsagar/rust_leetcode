// Time taken: 00:31, 00:34 -> Acc
struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result = String::new();
        let mut count = 0;
        let mut idx = 0;
        let s = s.chars().collect::<Vec<char>>();

        while idx < s.len() {
            if s[idx] == ' ' {
                count += 1;
                if count == k {
                    break;
                }
            }

            result.push(s[idx]);
            idx += 1;
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
            Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you"
        );
        assert_eq!(
            Solution::truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution"
        );
        assert_eq!(
            Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5),
            "chopper is not a tanuki"
        );
    }
}
