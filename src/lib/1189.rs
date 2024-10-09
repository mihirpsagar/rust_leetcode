use std::collections::HashMap;

// Time taken: 23:20, 23:30 -> Wrong, 23:33 -> Acc
struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map = HashMap::new();
        let chars = ['b', 'a', 'l', 'o', 'n'];
        let mut result = None;

        for ch in text.chars() {
            if chars.contains(&ch) {
                *map.entry(ch).or_insert(0) += 1;
            }
        }

        for ch in chars {
            if let Some(&val) = map.get(&ch) {
                if let Some(prev_val) = result {
                    if ch == 'l' || ch == 'o' {
                        result = Some(std::cmp::min(val / 2, prev_val));
                    } else {
                        result = Some(std::cmp::min(val, prev_val));
                    }
                } else {
                    result = Some(val);
                }
            } else {
                result = Some(0);
                break;
            }
        }

        if result.is_none() {
            return 0;
        } else {
            return result.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
        assert_eq!(Solution::max_number_of_balloons("balon".to_string()), 0);
    }
}
