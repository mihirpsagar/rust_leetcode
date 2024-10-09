// Time taken: 00:32, 00:37 -> Acc
struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut track = 0;
        let mut result = 0;

        let mut inc_ch = None;
        for ch in s.chars() {
            if track == 0 {
                track += 1;
                inc_ch = Some(ch);
            } else {
                if let Some(prev_ch) = inc_ch {
                    if ch == prev_ch {
                        track += 1;
                    } else {
                        track -= 1;
                        if track == 0 {
                            inc_ch = None;
                            result += 1;
                        }
                    }
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }
}
