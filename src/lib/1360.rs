// Time taken: 00:27, 00:44 -> Acc
struct Solution {}

impl Solution {
    pub fn days_from_1970(date: String) -> i32 {
        let mut result: i32 = 0;
        let year = u16::from_str_radix(&date[0..4], 10).unwrap();
        let month = u8::from_str_radix(&date[5..7], 10).unwrap();
        let day = u8::from_str_radix(&date[8..], 10).unwrap();

        for y in 1970..year {
            if y % 400 == 0 {
                result += 366;
            } else if y % 4 == 0 && y % 100 != 0 {
                result += 366;
            } else {
                result += 365;
            }
        }
        for m in 1..month {
            if m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12 {
                result += 31;
            } else if m == 2 {
                if year % 400 == 0 {
                    result += 29;
                } else if year % 4 == 0 && year % 100 != 0 {
                    result += 29;
                } else {
                    result += 28;
                }
            } else {
                result += 30;
            }
        }

        result += day as i32;
        return result;
    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let diff1 = Self::days_from_1970(date1);
        let diff2 = Self::days_from_1970(date2);

        return diff1.abs_diff(diff2) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
            1
        );
        assert_eq!(
            Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
            15
        );
    }
}
