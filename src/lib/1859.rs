// Time taken: 17:06, 17:11 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut sentence = vec![String::new(); 10];
        let s = s.chars().collect::<Vec<char>>();
        let zero_ascii = '0' as u8;
        let mut idx = 0;
        let mut word = String::new();

        while idx < s.len() {
            if s[idx].is_ascii_digit() {
                sentence[(s[idx] as u8 - zero_ascii - 1) as usize] = word.clone();
                word = String::new();
            } else {
                if s[idx] != ' ' {
                    word.push(s[idx]);
                }
            }
            idx += 1;
        }

        let mut result = String::new();
        for word in sentence {
            if word.is_empty() {
                break;
            }
            result.push_str(&word);
            result.push(' ');
        }

        result.pop();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_sentence("is2 sentence4 This1 a3".to_string()),
            "This is a sentence"
        );
        assert_eq!(
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I"
        );
    }
}
