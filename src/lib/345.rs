// Time taken: 21:01, 21:16 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut start = 0;
        let mut end = s.len() - 1;
        let mut result_vec: Vec<char> = s.clone().chars().collect();
        let mut vowels = vec!['a', 'e', 'i', 'o', 'u'];

        while start < end {
            let mut both_vowel = true;
            if !vowels.contains(&result_vec[start].to_ascii_lowercase()) {
                both_vowel = false;
                start += 1;
            }
            if !vowels.contains(&result_vec[end].to_ascii_lowercase()) {
                both_vowel = false;
                end -= 1;
            }

            if both_vowel {
                let tmp = result_vec[start];
                result_vec[start] = result_vec[end];
                result_vec[end] = tmp;

                start += 1;
                end -= 1;
            }
        }

        return result_vec.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_vowels(String::from("IceCreAm")),
            "AceCreIm"
        );
        assert_eq!(
            Solution::reverse_vowels(String::from("leetcode")),
            "leotcede"
        );
    }
}
