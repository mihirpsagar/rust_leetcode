// Time taken: 02:48, 02:58 -> Acc
struct Solution {}

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut result = 1;
        let time = time.chars().collect::<Vec<char>>();

        match (time[0], time[1]) {
            ('?', '?') => {
                result = 24;
            }
            ('?', '0' | '1' | '2' | '3') => {
                result = 3;
            }
            ('?', _) => {
                result = 2;
            }
            ('0' | '1', '?') => {
                result = 10;
            }
            ('2', '?') => {
                result = 4;
            }
            (_, _) => {}
        }

        match (time[3], time[4]) {
            ('?', '?') => {
                result *= 60;
            }
            ('?', _) => {
                result *= 6;
            }
            (_, '?') => {
                result *= 10;
            }
            (_, _) => {}
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}
