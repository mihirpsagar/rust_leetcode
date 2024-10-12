// Time taken: 14:13, 14:24 -> Wrong, 14:26 -> Acc
struct Solution {}

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let date = date.split_ascii_whitespace().collect::<Vec<&str>>();
        let day = date[0].chars().collect::<Vec<char>>();
        let mut res_day = String::new();

        if day[1].is_ascii_alphabetic() {
            res_day = String::from("0") + &day[0].to_string();
        } else {
            res_day = day[0..2].iter().collect::<String>();
        }

        let mut res_month = String::new();
        match date[1] {
            "Jan" => {
                res_month = String::from("01");
            }
            "Feb" => {
                res_month = String::from("02");
            }
            "Mar" => {
                res_month = String::from("03");
            }
            "Apr" => {
                res_month = String::from("04");
            }
            "May" => {
                res_month = String::from("05");
            }
            "Jun" => {
                res_month = String::from("06");
            }
            "Jul" => {
                res_month = String::from("07");
            }
            "Aug" => {
                res_month = String::from("08");
            }
            "Sep" => {
                res_month = String::from("09");
            }
            "Oct" => {
                res_month = String::from("10");
            }
            "Nov" => {
                res_month = String::from("11");
            }
            "Dec" => {
                res_month = String::from("12");
            }
            _ => {}
        }

        return date[2].to_string()
            + &String::from("-")
            + &res_month
            + &String::from("-")
            + &res_day;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reformat_date("20th Oct 2052".to_string()),
            "2052-10-20"
        );
        assert_eq!(
            Solution::reformat_date("6th Jun 1933".to_string()),
            "1933-06-06"
        );
        assert_eq!(
            Solution::reformat_date("26th May 1960".to_string()),
            "1960-05-26"
        );
        assert_eq!(
            Solution::reformat_date("3rd Jun 1998".to_string()),
            "1998-06-03"
        );
    }
}
