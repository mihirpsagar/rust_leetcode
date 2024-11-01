// Time taken: 21:06, 21:13, 21:18 -> Wrong, 21:19 -> Acc
struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut result = Vec::new();
        let title = title.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let mut prev_space = -1;

        while idx < title.len() {
            if prev_space + 1 == idx as i32 {
                result.push(title[idx].to_ascii_uppercase());
            } else if title[idx] == ' ' {
                if (idx as i32 - prev_space) <= 3 {
                    result[(prev_space + 1) as usize] =
                        result[(prev_space + 1) as usize].to_ascii_lowercase();
                }
                prev_space = idx as i32;
                result.push(' ');
            } else {
                result.push(title[idx].to_ascii_lowercase());
            }
            idx += 1;
        }
        if (idx as i32 - prev_space) <= 3 {
            result[(prev_space + 1) as usize] =
                result[(prev_space + 1) as usize].to_ascii_lowercase();
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
            Solution::capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title"
        );
        assert_eq!(
            Solution::capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word"
        );
        assert_eq!(
            Solution::capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode"
        );
        assert_eq!(Solution::capitalize_title("L hV".to_string()), "l hv");
    }
}
