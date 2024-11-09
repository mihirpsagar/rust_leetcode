// Time taken: 10:18, 10:21 -> Acc
struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let mut idx = left;
        let mut result = 0;
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        while idx <= right {
            let word = words[idx].chars().collect::<Vec<char>>();
            if vowels.contains(&word[0]) && vowels.contains(&word[word.len() - 1]) {
                result += 1;
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
            Solution::vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            ),
            2
        );
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            ),
            3
        );
    }
}
