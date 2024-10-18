// Time taken: 11:46, 12:07, 12:12 -> Acc, 12:14
struct Solution {}

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut words = Vec::new();
        let mut space_count = 0;
        let mut result = Vec::new();
        let text = text.chars().collect::<Vec<char>>();

        let mut str = String::new();
        for ch in text {
            if ch == ' ' {
                space_count += 1;
                if !str.is_empty() {
                    words.push(str);
                    str = String::new();
                }
            } else {
                str.push(ch);
            }
        }

        if !str.is_empty() {
            words.push(str);
        }

        let mut width = 0;
        if words.len() > 1 {
            width = space_count / (words.len() - 1);
        }
        let diff = space_count - (width * (words.len() - 1));

        let mut idx = 0;
        while idx < words.len() {
            for ch in words[idx].chars() {
                result.push(ch);
            }
            if idx != words.len() - 1 {
                for _ in 0..width {
                    result.push(' ');
                }
            }
            idx += 1;
        }

        for _ in 0..diff {
            result.push(' ');
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reorder_spaces("  this   is  a sentence ".to_string()),
            "this   is   a   sentence"
        );
        assert_eq!(
            Solution::reorder_spaces(" practice   makes   perfect".to_string()),
            "practice   makes   perfect "
        );
        assert_eq!(Solution::reorder_spaces("a".to_string()), "a");
    }
}
