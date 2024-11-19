// Time taken: 10:40, 10:45 -> Wrong, 10:46 -> Acc
struct Solution {}

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();

        match (s[0], s[1]) {
            ('?', '?') => {
                s[0] = '1';
                s[1] = '1';
            }
            (x, '?') => {
                if x == '1' {
                    s[1] = '1';
                } else {
                    s[1] = '9';
                }
            }
            ('?', x) => {
                if x == '0' || x == '1' {
                    s[0] = '1';
                } else {
                    s[0] = '0';
                }
            }
            (_, _) => {}
        }

        match (s[3], s[4]) {
            ('?', '?') => {
                s[3] = '5';
                s[4] = '9';
            }
            (_, '?') => {
                s[4] = '9';
            }
            ('?', _) => {
                s[3] = '5';
            }
            (_, _) => {}
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54");
        assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59");
        assert_eq!(Solution::find_latest_time("?1:?6".to_string()), "11:56");
        assert_eq!(Solution::find_latest_time("?0:40".to_string()), "10:40");
    }
}
