// Time taken: 11:25, 11:32, 11:39 -> Acc
struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut alpha: Vec<i32> = vec![-2; 26];
        let a = b'a' as usize;

        for (idx, ch) in s.chars().enumerate() {
            match alpha[ch as usize - a] {
                -2 => {
                    alpha[ch as usize - a] = idx as i32;
                }
                -1 => {}
                _ => {
                    alpha[ch as usize - a] = -1;
                }
            }
        }

        let mut result = None;
        for num in alpha {
            if num > -1 {
                match result {
                    None => {
                        result = Some(num);
                    }
                    Some(prev) => {
                        if prev > num {
                            result = Some(num);
                        }
                    }
                }
            }
        }

        if result.is_none() {
            result = Some(-1);
        }

        return result.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
        assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
        assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
    }
}
