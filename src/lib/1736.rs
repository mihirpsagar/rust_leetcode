// Time taken: 23:51, 23:56 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut result = String::new();
        let time = time.chars().collect::<Vec<char>>();

        match (time[0], time[1]) {
            ('?', '?') => {
                result.push('2');
                result.push('3');
            }
            ('?', ch) => {
                if ch == '0' || ch == '1' || ch == '2' || ch == '3' {
                    result.push('2');
                    result.push(ch);
                } else {
                    result.push('1');
                    result.push(ch);
                }
            }
            (ch, '?') => {
                if ch == '2' {
                    result.push(ch);
                    result.push('3');
                } else {
                    result.push(ch);
                    result.push('9');
                }
            }
            (ch1, ch2) => {
                result.push(ch1);
                result.push(ch2);
            }
        }

        result.push(':');

        match (time[3], time[4]) {
            ('?', '?') => {
                result.push('5');
                result.push('9');
            }
            ('?', ch) => {
                result.push('5');
                result.push(ch);
            }
            (ch, '?') => {
                result.push(ch);
                result.push('9');
            }
            (ch1, ch2) => {
                result.push(ch1);
                result.push(ch2);
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
        assert_eq!(Solution::maximum_time("2?:?0".to_string()), "23:50");
        assert_eq!(Solution::maximum_time("0?:3?".to_string()), "09:39");
        assert_eq!(Solution::maximum_time("1?:22".to_string()), "19:22");
    }
}
