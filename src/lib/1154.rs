// Time taken: 00:52, 01:09 -> Acc
struct Solution {}

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let year = usize::from_str_radix(&date[0..4], 10).unwrap();
        let month = usize::from_str_radix(&date[5..7], 10).unwrap();
        let date = usize::from_str_radix(&date[8..], 10).unwrap();
        let mut result = 0;
        let mut day_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    day_month[1] += 1;
                }
            } else {
                day_month[1] += 1;
            }
        }

        let mut curr_month = 1;

        while curr_month < month {
            result += day_month[curr_month - 1];
            curr_month += 1;
        }

        result += date;

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    }
}
