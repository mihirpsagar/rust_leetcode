// Time taken: 19:26, 19:38 -> Acc
struct Solution {}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev_num = None;
        let s = s.chars().collect::<Vec<char>>();
        let mut idx = 0;

        let mut num = Vec::new();
        while idx < s.len() {
            if s[idx].is_ascii_digit() {
                num.push(s[idx]);
            } else {
                if !num.is_empty() {
                    if !Self::is_increasing(&prev_num, &num) {
                        return false;
                    }
                    prev_num = Some(num);
                    num = Vec::new();
                }
            }
            idx += 1;
        }

        if !num.is_empty() {
            return Self::is_increasing(&prev_num, &num);
        }

        return true;
    }

    pub fn is_increasing(prev_num: &Option<Vec<char>>, num: &Vec<char>) -> bool {
        if prev_num.is_none() {
            return true;
        }
        let prev_num = prev_num.as_ref().unwrap();

        if num.len() > prev_num.len() {
            return true;
        }

        if num.len() < prev_num.len() {
            return false;
        }

        let mut idx = 0;

        while idx < prev_num.len() {
            if prev_num[idx] < num[idx] {
                return true;
            } else if prev_num[idx] > num[idx] {
                return false;
            } else {
                idx += 1;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
        assert_eq!(
            Solution::are_numbers_ascending(
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
            ),
            false
        );
    }
}
