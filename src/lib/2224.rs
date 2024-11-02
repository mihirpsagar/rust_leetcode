// Time taken: 00:43, 00:57, 01:07 -> Acc
struct Solution {}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let mut time1 = 0;
        let mut time2 = 0;
        let max_time: u32 = 60 * 24;
        let mut diff = 0;
        let mut result = 0;

        time1 = time1 + (u32::from_str_radix(&current[0..2], 10).unwrap() * 60);
        time2 = time2 + (u32::from_str_radix(&correct[0..2], 10).unwrap() * 60);

        time1 = time1 + u32::from_str_radix(&current[3..], 10).unwrap();
        time2 = time2 + u32::from_str_radix(&correct[3..], 10).unwrap();

        if time1 == time2 {
            return 0;
        }

        if time1 < time2 {
            diff = time2 - time1;
        } else {
            diff = (max_time - time1) + time2;
        }

        if diff >= 60 {
            result = result + (diff / 60);
            diff = diff % 60;
        }

        if diff >= 15 {
            result = result + (diff / 15);
            diff = diff % 15;
        }

        if diff >= 5 {
            result = result + (diff / 5);
            diff = diff % 5;
        }

        if diff > 0 {
            result += diff;
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
            Solution::convert_time("02:30".to_string(), "04:35".to_string()),
            3
        );
        assert_eq!(
            Solution::convert_time("11:00".to_string(), "11:01".to_string()),
            1
        );
        assert_eq!(
            Solution::convert_time("16:00".to_string(), "15:00".to_string()),
            23
        );
    }
}
