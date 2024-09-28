use std::collections::HashSet;

// Time taken: 01:15, 01:20 -> Acc
struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let a_ascii = 'a' as u8;

        let mut set = HashSet::new();

        for word in words {
            let mut str = String::new();
            for ch in word.chars() {
                str += &morse[(ch as u8 - a_ascii) as usize];
            }
            set.insert(str);
        }

        return set.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::unique_morse_representations(vec!["a".to_string()]),
            1
        );
    }
}
