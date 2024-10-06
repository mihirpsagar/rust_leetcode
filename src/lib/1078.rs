// Time taken: 21:04, 21:08
struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text = text.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < text.len() {
            if text[idx] == first && idx + 2 < text.len() {
                if text[idx + 1] == second {
                    result.push(text[idx + 2].to_string());
                }
            }
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
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string()
            ),
            ["girl", "student"]
        );
        assert_eq!(
            Solution::find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string()
            ),
            ["we", "rock"]
        );
    }
}
