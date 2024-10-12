// Time taken: 18:08, 18:09, 18:21 -> Acc
struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut result = vec!['a'; s.len()];
        let mut idx = 0;
        while idx < s.len() {
            result[indices[idx] as usize] = s[idx];
            idx += 1;
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
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode"
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc"
        );
    }
}
