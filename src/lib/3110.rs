// Time taken: 10:38, 10:39 -> Acc
struct Solution {}

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut sum = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;

        while idx < s.len() - 1 {
            let val = (s[idx] as u8).abs_diff(s[idx + 1] as u8);
            sum = sum + val as i32;

            idx += 1;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
        assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
    }
}
