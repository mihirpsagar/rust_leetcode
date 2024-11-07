// Time taken: 03:06, 03:11 -> Acc
struct Solution {}

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let event1_start = Self::get_time(&event1[0]);
        let event1_end = Self::get_time(&event1[1]);
        let event2_start = Self::get_time(&event2[0]);
        let event2_end = Self::get_time(&event2[1]);

        if (event2_start >= event1_start && event2_start <= event1_end)
            || (event1_start >= event2_start && event1_start <= event2_end)
        {
            return true;
        }

        return false;
    }

    pub fn get_time(time: &String) -> u16 {
        let hh = u16::from_str_radix(&time[0..2], 10).unwrap();
        let mm = u16::from_str_radix(&time[3..], 10).unwrap();

        let mut result = 60 * hh;
        result += mm;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::have_conflict(
                vec!["01:15".to_string(), "02:00".to_string()],
                vec!["02:00".to_string(), "03:00".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::have_conflict(
                vec!["01:00".to_string(), "02:00".to_string()],
                vec!["01:20".to_string(), "03:00".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::have_conflict(
                vec!["10:00".to_string(), "11:00".to_string()],
                vec!["14:00".to_string(), "15:00".to_string()]
            ),
            false
        );
    }
}
