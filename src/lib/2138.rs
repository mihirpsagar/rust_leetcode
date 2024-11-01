// Time taken: 21:28, 21:32, 21:34 -> Acc
struct Solution {}

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut result = Vec::new();
        let k = k as usize;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;

        while idx < s.len() {
            let mut idx2 = idx;
            let mut word = Vec::new();

            for _ in 0..k {
                if idx2 >= s.len() {
                    word.push(fill);
                } else {
                    word.push(s[idx2]);
                }
                idx2 += 1;
            }

            result.push(word.iter().collect());
            idx = idx + k;
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
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            ["abc", "def", "ghi"]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            ["abc", "def", "ghi", "jxx"]
        );
    }
}
