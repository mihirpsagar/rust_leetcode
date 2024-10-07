// Time taken: 01:11, 01:16 -> Acc
struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count = vec![0; 26];
        let a_ascii = 'a' as u8;
        let mut result = 0;

        for ch in chars.chars() {
            count[(ch as u8 - a_ascii) as usize] += 1;
        }

        'outer: for word in words {
            let mut curr_count = count.clone();
            for ch in word.chars() {
                let idx = (ch as u8 - a_ascii) as usize;
                if curr_count[idx] == 0 {
                    continue 'outer;
                } else {
                    curr_count[idx] -= 1;
                }
            }
            result += word.len();
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
