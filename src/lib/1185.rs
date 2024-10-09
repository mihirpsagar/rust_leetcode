// Time taken: 22:54, 23:08 -> Wrong, 23:20 -> Acc
struct Solution {}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut result = 3; // 31st Dec 1970 - Thursday [0 Mon - 6 Sun]

        for num in 1971..year {
            if num % 400 == 0 {
                result = (result + 366) % 7;
            } else if num % 4 == 0 && num % 100 != 0 {
                result = (result + 366) % 7;
            } else {
                result = (result + 365) % 7;
            }
        }

        for num in 1..month {
            if num == 1 || num == 3 || num == 5 || num == 7 || num == 8 || num == 10 || num == 12 {
                result = (result + 31) % 7;
            } else if num == 2 {
                if year % 400 == 0 {
                    result = (result + 29) % 7;
                } else if year % 4 == 0 && year % 100 != 0 {
                    result = (result + 29) % 7;
                } else {
                    result = (result + 28) % 7;
                }
            } else {
                result = (result + 30) % 7;
            }
        }

        result = (result + day) % 7;

        if result == 0 {
            return String::from("Monday");
        } else if result == 1 {
            return String::from("Tuesday");
        } else if result == 2 {
            return String::from("Wednesday");
        } else if result == 3 {
            return String::from("Thursday");
        } else if result == 4 {
            return String::from("Friday");
        } else if result == 5 {
            return String::from("Saturday");
        } else {
            return String::from("Sunday");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
        assert_eq!(Solution::day_of_the_week(21, 12, 1980), "Sunday");
    }
}
